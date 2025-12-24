use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Ord for FixedBitSet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.length
            .cmp(&other.length)
            .then_with(|| self.as_simd_slice().cmp(other.as_simd_slice()))
    }
}
