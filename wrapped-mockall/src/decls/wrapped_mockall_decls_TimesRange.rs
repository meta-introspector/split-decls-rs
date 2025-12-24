use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug)]
#[doc(hidden)]
pub struct TimesRange(Range<usize>);
