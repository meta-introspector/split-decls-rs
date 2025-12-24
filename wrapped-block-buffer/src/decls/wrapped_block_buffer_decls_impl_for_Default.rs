use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<BS: ArraySize, K: BufferKind> Default for BlockBuffer<BS, K> {
    #[inline]
    fn default() -> Self {
        assert!(Self::BLOCK_SIZE_ASSERT);
        let mut buffer = MaybeUninit::uninit();
        let mut pos = Default::default();
        K::set_pos(&mut buffer, &mut pos, 0);
        Self { buffer, pos }
    }
}
