use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub trait GetSpan<T>: crate::sealed::Sealed<T> {
    fn span_for(&self, target: &T) -> tracing::Span;
}
