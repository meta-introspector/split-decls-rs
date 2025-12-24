use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl From<RangeInclusive<usize>> for TimesRange {
    fn from(r: RangeInclusive<usize>) -> TimesRange {
        assert!(r.end() >= r.start(), "Backwards range");
        TimesRange(*r.start()..*r.end() + 1)
    }
}
