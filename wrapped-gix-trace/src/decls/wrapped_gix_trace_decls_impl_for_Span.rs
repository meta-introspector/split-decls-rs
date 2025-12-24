use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Span {
    /// Execute `f` in with this span active, consuming it.
    pub fn into_scope<T>(self, f: impl FnOnce() -> T) -> T {
        f()
    }
}
