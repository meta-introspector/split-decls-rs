use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Default for Uuid {
    #[inline]
    fn default() -> Self {
        Uuid::nil()
    }
}
