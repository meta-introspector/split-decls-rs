// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_trait_selection/src/error_reporting/traits/suggestions.rs
// Error: expected `,`
// Problematic line: line 13

use rustc_data_structures::fx::FxHashSet;
use rustc_data_structures::stack::ensure_sufficient_stack;
use rustc_errors::codes::*;
use rustc_errors::{
    Applicability, Diag, EmissionGuarantee, MultiSpan, Style, SuggestionStyle, pluralize,
    struct_span_code_err,
};
