// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_ast_passes/src/ast_validation.rs
// Error: expected square brackets
// Problematic line: line 34

use rustc_feature::Features;
use rustc_session::Session;
use rustc_session::lint::BuiltinLintDiag;
use rustc_session::lint::builtin::{
    DEPRECATED_WHERE_CLAUSE_LOCATION, MISSING_ABI, MISSING_UNSAFE_ON_EXTERN,
    PATTERNS_IN_FNS_WITHOUT_BODY,
};
