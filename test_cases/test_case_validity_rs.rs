// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_const_eval/src/interpret/validity.rs
// Error: expected square brackets
// Problematic line: line 14


use either::{Left, Right};
use hir::def::DefKind;
use rustc_abi::{
    BackendRepr, FieldIdx, FieldsShape, Scalar as ScalarAbi, Size, VariantIdx, Variants,
    WrappingRange,
};
