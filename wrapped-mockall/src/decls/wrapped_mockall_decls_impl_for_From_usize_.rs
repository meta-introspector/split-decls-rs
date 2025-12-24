use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl From<usize> for TimesRange {
    fn from(n: usize) -> TimesRange {
        TimesRange(n..(n + 1))
    }
}
