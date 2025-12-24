use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An iterator producing elements in the symmetric difference of two sets.
///
/// This struct is created by the [`FixedBitSet::symmetric_difference`] method.
pub struct SymmetricDifference<'a> {
    iter: Chain<Difference<'a>, Difference<'a>>,
}
