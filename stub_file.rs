pub mod reg {
    pub struct Stub;
}


// SRC: ../rust/compiler/rustc_abi/src/callconv.rs
#[cfg(feature = "nightly")]
use crate::{BackendRepr, FieldsShape, Primitive, Size, TyAbiInterface, TyAndLayout, Variants};


pub use reg::{Reg, RegKind};

/// Return value from the `homogeneous_aggregate` test function.

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum HomogeneousAggregate { Stub }

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Heterogeneous;;

impl HomogeneousAggregate {}

