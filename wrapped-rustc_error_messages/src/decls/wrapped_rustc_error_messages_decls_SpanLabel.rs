use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A span together with some additional data.
#[derive(Clone, Debug)]
pub struct SpanLabel {
    /// The span we are going to include in the final snippet.
    pub span: Span,
    /// Is this a primary span? This is the "locus" of the message,
    /// and is indicated with a `^^^^` underline, versus `----`.
    pub is_primary: bool,
    /// What label should we attach to this span (if any)?
    pub label: Option<DiagMessage>,
}
