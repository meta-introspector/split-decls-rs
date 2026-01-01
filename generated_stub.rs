pub mod reg {
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct Reg;

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct RegKind;

}


// SRC: ../rust/compiler/rustc_abi/src/callconv.rs
#[cfg(feature = "nightly")]
use crate::{BackendRepr, FieldsShape, Primitive, Size, TyAbiInterface, TyAndLayout, Variants};


pub use reg::{Reg, RegKind};

/// Return value from the `homogeneous_aggregate` test function.

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum HomogeneousAggregate { Stub }

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Heterogeneous;

impl HomogeneousAggregate {}

pub fn unit() {}

pub fn homogeneous_aggregate<C>() {}

fn main() {}
