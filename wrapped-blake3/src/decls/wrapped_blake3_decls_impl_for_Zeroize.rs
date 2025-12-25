use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "zeroize")]
impl Zeroize for OutputReader {
    fn zeroize(&mut self) {
        let Self { inner, position_within_block } = self;
        inner.zeroize();
        position_within_block.zeroize();
    }
}
