use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl ops::AddAssign<ByteSize> for ByteSize {
    #[inline(always)]
    fn add_assign(&mut self, rhs: ByteSize) {
        self.0 += rhs.0;
    }
}
