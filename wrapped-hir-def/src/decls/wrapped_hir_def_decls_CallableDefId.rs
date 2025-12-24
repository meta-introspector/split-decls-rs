use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
pub enum CallableDefId {
    FunctionId(FunctionId),
    StructId(StructId),
    EnumVariantId(EnumVariantId),
}
