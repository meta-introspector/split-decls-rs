use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Drop for FixedBitSet {
    fn drop(&mut self) {
        drop(unsafe {
            Vec::from_raw_parts(self.data.as_ptr(), self.simd_block_len(), self.capacity)
        });
    }
}
