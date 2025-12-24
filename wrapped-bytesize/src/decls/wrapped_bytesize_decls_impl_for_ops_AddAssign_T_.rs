use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<T> ops::AddAssign<T> for ByteSize
where
    T: Into<u64>,
{
    #[inline(always)]
    fn add_assign(&mut self, rhs: T) {
        self.0 += rhs.into();
    }
}
