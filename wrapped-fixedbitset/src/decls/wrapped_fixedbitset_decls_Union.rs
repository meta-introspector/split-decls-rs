use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An iterator producing elements in the union of two sets.
///
/// This struct is created by the [`FixedBitSet::union`] method.
pub struct Union<'a> {
    iter: Chain<Ones<'a>, Difference<'a>>,
}
