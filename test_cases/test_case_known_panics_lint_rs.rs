// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_transform/src/known_panics_lint.rs
// Error: expected square brackets
// Problematic line: line 9


use rustc_abi::{BackendRepr, FieldIdx, HasDataLayout, Size, TargetDataLayout, VariantIdx};
use rustc_const_eval::const_eval::DummyMachine;
use rustc_const_eval::interpret::{
    ImmTy, InterpCx, InterpResult, Projectable, Scalar, format_interp_error, interp_ok,
};
use rustc_data_structures::fx::FxHashSet;
