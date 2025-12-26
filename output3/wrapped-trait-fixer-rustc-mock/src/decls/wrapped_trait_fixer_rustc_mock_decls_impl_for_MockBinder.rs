use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl MockBinder {
    pub fn dummy<T>(_value: T) -> MockBinder {
        MockBinder
    }
}
