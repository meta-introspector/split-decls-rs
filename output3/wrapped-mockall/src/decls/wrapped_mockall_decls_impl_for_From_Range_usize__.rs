use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl From<Range<usize>> for TimesRange {
    fn from(r: Range<usize>) -> TimesRange {
        assert!(r.end > r.start, "Backwards range");
        TimesRange(r)
    }
}
