use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl ColorChoice {
    /// Get the current [`ColorChoice`] state
    pub fn global() -> Self {
        USER.get()
    }
    /// Override the detected [`ColorChoice`]
    pub fn write_global(self) {
        USER.set(self);
    }
}
