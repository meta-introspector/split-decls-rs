use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T: Default, E> Default for ValueResult<T, E> {
    fn default() -> Self {
        Self {
            value: Default::default(),
            err: Default::default(),
        }
    }
}
