#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    warnings,
    unused,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
#![cfg_attr(use_asm, feature(asm))]
#![cfg_attr(not(use_asm), forbid(unsafe_code))]
#![cfg_attr(use_asm, deny(unsafe_code))]

#[macro_use]
extern crate ark_std;

#[macro_use]
extern crate derivative;

#[cfg_attr(test, macro_use)]
pub mod bytes;
pub use self::bytes::*;

#[macro_use]
pub mod fields;
pub use self::fields::*;

pub mod biginteger;
pub use self::biginteger::*;

mod rand;
pub use self::rand::*;

mod to_field_vec;
pub use to_field_vec::ToConstraintField;

pub use num_traits::{One, Zero};

pub use ark_std::vec;

pub mod prelude {
    pub use crate::biginteger::BigInteger;

    pub use crate::fields::{Field, FpParameters, PrimeField, SquareRootField};

    pub use crate::rand::UniformRand;

    pub use num_traits::{One, Zero};
}

#[cfg(not(feature = "std"))]
fn error(_msg: &'static str) -> ark_std::io::Error {
    ark_std::io::Error
}

#[cfg(feature = "std")]
fn error(msg: &'static str) -> ark_std::io::Error {
    ark_std::io::Error::new(ark_std::io::ErrorKind::Other, msg)
}
