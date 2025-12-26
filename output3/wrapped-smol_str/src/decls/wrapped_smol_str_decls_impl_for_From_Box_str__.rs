use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl From<Box<str>> for SmolStr {
    #[inline]
    fn from(s: Box<str>) -> SmolStr {
        SmolStr::new(s)
    }
}
