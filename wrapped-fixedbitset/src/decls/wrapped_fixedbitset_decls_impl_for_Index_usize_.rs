use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Return **true** if the bit is enabled in the bitset,
/// or **false** otherwise.
///
/// Note: bits outside the capacity are always disabled, and thus
/// indexing a FixedBitSet will not panic.
impl Index<usize> for FixedBitSet {
    type Output = bool;
    #[inline]
    fn index(&self, bit: usize) -> &bool {
        if self.contains(bit) { &true } else { &false }
    }
}
