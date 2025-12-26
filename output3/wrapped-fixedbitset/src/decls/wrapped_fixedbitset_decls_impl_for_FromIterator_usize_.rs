use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Return a FixedBitSet containing bits set to **true** for every bit index in
/// the iterator, other bits are set to **false**.
impl FromIterator<usize> for FixedBitSet {
    fn from_iter<I: IntoIterator<Item = usize>>(src: I) -> Self {
        let mut fbs = FixedBitSet::with_capacity(0);
        fbs.extend(src);
        fbs
    }
}
