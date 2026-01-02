// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_span/src/hygiene.rs
// Error: expected square brackets
// Problematic line: line 48

use crate::symbol::{Symbol, kw, sym};
use crate::{DUMMY_SP, HashStableContext, Span, SpanDecoder, SpanEncoder, with_session_globals};

/// A `SyntaxContext` represents a chain of pairs `(ExpnId, Transparency)` named "marks".
///
/// See <https://rustc-dev-guide.rust-lang.org/macro-expansion.html> for more explanation.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
