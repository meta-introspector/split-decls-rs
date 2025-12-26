use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Integers, also used for enum discriminants.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[cfg_attr(
    feature = "nightly",
    derive(Encodable_NoContext, Decodable_NoContext, HashStable_Generic)
)]
pub enum Integer {
    I8,
    I16,
    I32,
    I64,
    I128,
}
