// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_hir_typeck/src/expr.rs
// Error: expected square brackets
// Problematic line: line 14

use rustc_data_structures::stack::ensure_sufficient_stack;
use rustc_data_structures::unord::UnordMap;
use rustc_errors::codes::*;
use rustc_errors::{
    Applicability, Diag, ErrorGuaranteed, MultiSpan, StashKey, Subdiagnostic, listify, pluralize,
    struct_span_code_err,
};
