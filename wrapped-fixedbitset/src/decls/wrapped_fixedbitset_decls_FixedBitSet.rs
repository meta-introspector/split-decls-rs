use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// `FixedBitSet` is a simple fixed size set of bits that each can
/// be enabled (1 / **true**) or disabled (0 / **false**).
///
/// The bit set has a fixed capacity in terms of enabling bits (and the
/// capacity can grow using the `grow` method).
///
/// Derived traits depend on both the zeros and ones, so [0,1] is not equal to
/// [0,1,0].
#[derive(Debug, Eq)]
pub struct FixedBitSet {
    pub(crate) data: NonNull<MaybeUninit<SimdBlock>>,
    capacity: usize,
    /// length in bits
    pub(crate) length: usize,
}
