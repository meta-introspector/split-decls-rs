use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl From<RangeTo<usize>> for TimesRange {
    fn from(r: RangeTo<usize>) -> TimesRange {
        TimesRange(0..r.end)
    }
}
