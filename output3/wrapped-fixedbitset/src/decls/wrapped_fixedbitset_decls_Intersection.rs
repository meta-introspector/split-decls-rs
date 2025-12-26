use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An iterator producing elements in the intersection of two sets.
///
/// This struct is created by the [`FixedBitSet::intersection`] method.
pub struct Intersection<'a> {
    iter: Ones<'a>,
    other: &'a FixedBitSet,
}
