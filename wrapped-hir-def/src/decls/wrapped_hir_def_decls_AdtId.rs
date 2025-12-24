use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A Data Type
#[derive(
    Debug,
    PartialOrd,
    Ord,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    salsa_macros::Supertype
)]
pub enum AdtId {
    StructId(StructId),
    UnionId(UnionId),
    EnumId(EnumId),
}
