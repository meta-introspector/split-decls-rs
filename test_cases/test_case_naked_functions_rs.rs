// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_hir_typeck/src/naked_functions.rs
// Error: expected square brackets
// Problematic line: line 12

use rustc_middle::ty::TyCtxt;
use rustc_span::Span;

use crate::errors::{
    NakedFunctionsAsmBlock, NakedFunctionsMustNakedAsm, NoPatterns, ParamsNotAllowed,
};

