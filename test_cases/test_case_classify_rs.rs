// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_ast/src/util/classify.rs
// Error: expected square brackets
// Problematic line: line 7

use crate::ast::{self, MatchKind};
use crate::token::Delimiter;

/// This classification determines whether various syntactic positions break out
/// of parsing the current expression (true) or continue parsing more of the
/// same expression (false).
///
