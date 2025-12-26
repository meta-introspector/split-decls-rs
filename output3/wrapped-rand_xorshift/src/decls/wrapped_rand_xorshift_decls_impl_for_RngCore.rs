use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl RngCore for XorShiftRng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        let x = self.x;
        let t = x ^ (x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        let w_ = self.w;
        self.w = w_ ^ (w_ >> 19) ^ (t ^ (t >> 8));
        self.w.0
    }
    #[inline]
    fn next_u64(&mut self) -> u64 {
        le::next_u64_via_u32(self)
    }
    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        le::fill_bytes_via_next(self, dest)
    }
}
