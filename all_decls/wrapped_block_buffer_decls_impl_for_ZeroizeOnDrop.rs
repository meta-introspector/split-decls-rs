use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "zeroize")]
impl<BS: ArraySize, K: BufferKind> ZeroizeOnDrop for BlockBuffer<BS, K> {}
