// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_const_eval/src/check_consts/ops.rs
// Error: expected square brackets
// Problematic line: line 13

use rustc_middle::mir::CallSource;
use rustc_middle::span_bug;
use rustc_middle::ty::print::{PrintTraitRefExt as _, with_no_trimmed_paths};
use rustc_middle::ty::{
    self, Closure, FnDef, FnPtr, GenericArgKind, GenericArgsRef, Param, TraitRef, Ty,
    suggest_constraining_type_param,
};
