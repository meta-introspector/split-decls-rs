use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T, I: iter::Iterator<Item = T>> From<I> for IntoFallible<I> {
    fn from(value: I) -> Self {
        Self(value)
    }
}
