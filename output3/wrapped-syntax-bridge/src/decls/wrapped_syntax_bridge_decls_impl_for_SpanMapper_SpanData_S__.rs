use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<S> SpanMapper<SpanData<S>> for SpanMap<S>
where
    SpanData<S>: Copy,
{
    fn span_for(&self, range: TextRange) -> SpanData<S> {
        self.span_at(range.start())
    }
}
