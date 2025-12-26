use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl PartialEq for Encoding {
    #[inline]
    fn eq(&self, other: &Encoding) -> bool {
        (self as *const Encoding) == (other as *const Encoding)
    }
}
