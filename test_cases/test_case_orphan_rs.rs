// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_hir_analysis/src/coherence/orphan.rs
// Error: expected square brackets
// Problematic line: line 8

use rustc_errors::ErrorGuaranteed;
use rustc_infer::infer::{DefineOpaqueTypes, InferCtxt, TyCtxtInferExt};
use rustc_lint_defs::builtin::UNCOVERED_PARAM_IN_PROJECTION;
use rustc_middle::ty::{
    self, Ty, TyCtxt, TypeSuperVisitable, TypeVisitable, TypeVisitableExt, TypeVisitor, TypingMode,
};
use rustc_middle::{bug, span_bug};
