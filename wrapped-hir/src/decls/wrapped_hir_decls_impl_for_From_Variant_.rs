use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl From<&Variant> for DefWithBodyId {
    fn from(&v: &Variant) -> Self {
        DefWithBodyId::VariantId(v.into())
    }
}
