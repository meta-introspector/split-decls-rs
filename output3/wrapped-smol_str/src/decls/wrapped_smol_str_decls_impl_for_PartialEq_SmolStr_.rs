use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl PartialEq<SmolStr> for &String {
    #[inline(always)]
    fn eq(&self, other: &SmolStr) -> bool {
        *self == other
    }
}
