use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl DerefMut for MmapMut {
    #[inline]
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.inner.mut_ptr(), self.inner.len()) }
    }
}
