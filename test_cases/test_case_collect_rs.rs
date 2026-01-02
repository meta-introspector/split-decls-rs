// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_hir_analysis/src/collect.rs
// Error: expected square brackets
// Problematic line: line 26

use rustc_ast::Recovered;
use rustc_data_structures::fx::{FxHashSet, FxIndexMap};
use rustc_data_structures::unord::UnordMap;
use rustc_errors::{
    Applicability, Diag, DiagCtxtHandle, E0228, ErrorGuaranteed, StashKey, struct_span_code_err,
};
use rustc_hir::attrs::AttributeKind;
