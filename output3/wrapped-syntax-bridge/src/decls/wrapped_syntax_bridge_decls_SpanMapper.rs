use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub trait SpanMapper<S> {
    fn span_for(&self, range: TextRange) -> S;
}
