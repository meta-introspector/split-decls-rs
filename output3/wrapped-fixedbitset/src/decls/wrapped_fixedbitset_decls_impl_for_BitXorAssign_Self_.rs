use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl BitXorAssign<&Self> for FixedBitSet {
    fn bitxor_assign(&mut self, other: &Self) {
        self.symmetric_difference_with(other);
    }
}
