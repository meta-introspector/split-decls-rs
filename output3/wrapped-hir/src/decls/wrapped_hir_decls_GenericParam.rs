use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GenericParam {
    TypeParam(TypeParam),
    ConstParam(ConstParam),
    LifetimeParam(LifetimeParam),
}
