// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_hir_typeck/src/coercion.rs
// Error: expected square brackets
// Problematic line: line 48

use rustc_hir_analysis::hir_ty_lowering::HirTyLowerer;
use rustc_infer::infer::relate::RelateResult;
use rustc_infer::infer::{DefineOpaqueTypes, InferOk, InferResult, RegionVariableOrigin};
use rustc_infer::traits::{
    MatchExpressionArmCause, Obligation, PredicateObligation, PredicateObligations, SelectionError,
};
use rustc_middle::span_bug;
