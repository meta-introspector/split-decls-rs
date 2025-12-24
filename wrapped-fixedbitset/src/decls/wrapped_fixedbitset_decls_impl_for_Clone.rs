use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Clone for FixedBitSet {
    #[inline]
    fn clone(&self) -> Self {
        Self::from_blocks_and_len(Vec::from(self.as_simd_slice()), self.length)
    }
    #[inline]
    fn clone_from(&mut self, source: &Self) {
        if self.length < source.length {
            unsafe { self.grow_inner(source.length, MaybeUninit::uninit()) };
        }
        let me = self.as_mut_simd_slice_uninit();
        let them = source.as_simd_slice_uninit();
        match me.len().cmp(&them.len()) {
            Ordering::Greater => {
                let (head, tail) = me.split_at_mut(them.len());
                head.copy_from_slice(them);
                tail.fill(MaybeUninit::new(SimdBlock::NONE));
            }
            Ordering::Equal => me.copy_from_slice(them),
            Ordering::Less => {}
        }
        self.length = source.length;
    }
}
