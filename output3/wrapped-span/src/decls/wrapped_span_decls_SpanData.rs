use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Spans represent a region of code, used by the IDE to be able link macro inputs and outputs
/// together. Positions in spans are relative to some [`SpanAnchor`] to make them more incremental
/// friendly.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpanData<Ctx> {
    /// The text range of this span, relative to the anchor.
    /// We need the anchor for incrementality, as storing absolute ranges will require
    /// recomputation on every change in a file at all times.
    pub range: TextRange,
    /// The anchor this span is relative to.
    pub anchor: SpanAnchor,
    /// The syntax context of the span.
    pub ctx: Ctx,
}
