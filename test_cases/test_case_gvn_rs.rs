// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_transform/src/gvn.rs
// Error: expected square brackets
// Problematic line: line 93

use itertools::Itertools as _;
use rustc_abi::{self as abi, BackendRepr, FIRST_VARIANT, FieldIdx, Primitive, Size, VariantIdx};
use rustc_const_eval::const_eval::DummyMachine;
use rustc_const_eval::interpret::{
    ImmTy, Immediate, InterpCx, MemPlaceMeta, MemoryKind, OpTy, Projectable, Scalar,
    intern_const_alloc_for_constprop,
};
