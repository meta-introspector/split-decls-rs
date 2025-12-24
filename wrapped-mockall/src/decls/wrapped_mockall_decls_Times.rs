use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Default)]
#[doc(hidden)]
pub struct Times {
    /// How many times has the expectation already been called?
    count: AtomicUsize,
    range: TimesRange,
}
