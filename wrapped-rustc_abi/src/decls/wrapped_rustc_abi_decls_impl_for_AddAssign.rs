use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl AddAssign for Size {
    #[inline]
    fn add_assign(&mut self, other: Size) {
        *self = *self + other;
    }
}
