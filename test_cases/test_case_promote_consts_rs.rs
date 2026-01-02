// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_transform/src/promote_consts.rs
// Error: expected square brackets
// Problematic line: line 30

use rustc_span::source_map::Spanned;
use tracing::{debug, instrument};

/// A `MirPass` for promotion.
///
/// Promotion is the extraction of promotable temps into separate MIR bodies so they can have
/// `'static` lifetime.
