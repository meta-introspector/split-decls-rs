use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, salsa_macros::Supertype)]
pub enum VariantId {
    EnumVariantId(EnumVariantId),
    StructId(StructId),
    UnionId(UnionId),
}
