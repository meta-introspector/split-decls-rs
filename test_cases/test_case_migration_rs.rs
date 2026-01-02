// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_build/src/thir/pattern/migration.rs
// Error: expected square brackets
// Problematic line: line 13

use crate::errors::{Rust2024IncompatiblePat, Rust2024IncompatiblePatSugg};
use crate::fluent_generated as fluent;

/// For patterns flagged for migration during HIR typeck, this handles constructing and emitting
/// a diagnostic suggestion.
pub(super) struct PatMigration<'a> {
    suggestion: Vec<(Span, String)>,
