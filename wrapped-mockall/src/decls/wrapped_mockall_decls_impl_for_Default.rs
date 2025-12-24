use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Default for TimesRange {
    fn default() -> TimesRange {
        TimesRange(0..usize::MAX)
    }
}
