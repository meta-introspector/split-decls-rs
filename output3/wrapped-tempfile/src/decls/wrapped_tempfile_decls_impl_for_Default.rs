use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Default for Builder<'_, '_> {
    fn default() -> Self {
        Builder {
            random_len: crate::NUM_RAND_CHARS,
            prefix: OsStr::new(".tmp"),
            suffix: OsStr::new(""),
            append: false,
            permissions: None,
            disable_cleanup: false,
        }
    }
}
