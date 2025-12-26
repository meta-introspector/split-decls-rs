use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl InnerSpan {
    pub fn new(start: usize, end: usize) -> InnerSpan {
        InnerSpan { start, end }
    }
}
