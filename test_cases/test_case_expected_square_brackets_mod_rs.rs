// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_hir_analysis/src/hir_ty_lowering/mod.rs
// Error: expected square brackets
// Error type: expected_square_brackets
// Sample #2 of 3
// Problematic line: line 29

use rustc_ast::TraitObjectSyntax;
use rustc_data_structures::fx::{FxHashSet, FxIndexMap, FxIndexSet};
use rustc_errors::codes::*;
use rustc_errors::{
    Applicability, Diag, DiagCtxtHandle, ErrorGuaranteed, FatalError, struct_span_code_err,
};
use rustc_hir::def::{CtorKind, CtorOf, DefKind, Res};
