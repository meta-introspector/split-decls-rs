use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl From<RangeFull> for TimesRange {
    fn from(_: RangeFull) -> TimesRange {
        TimesRange(0..usize::MAX)
    }
}
