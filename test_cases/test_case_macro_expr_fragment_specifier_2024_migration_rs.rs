// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_lint/src/macro_expr_fragment_specifier_2024_migration.rs
// Error: expected square brackets
// Problematic line: line 14

use crate::EarlyLintPass;
use crate::lints::MacroExprFragment2024;

declare_lint! {
    /// The `edition_2024_expr_fragment_specifier` lint detects the use of
    /// `expr` fragments in macros during migration to the 2024 edition.
    ///
