use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<BS: ArraySize, K: BufferKind> Drop for BlockBuffer<BS, K> {
    #[inline]
    fn drop(&mut self) {
        #[cfg(feature = "zeroize")]
        self.zeroize();
    }
}
