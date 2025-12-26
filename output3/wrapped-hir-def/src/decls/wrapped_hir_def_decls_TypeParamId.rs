use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A TypeOrConstParamId with an invariant that it actually belongs to a type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeParamId(TypeOrConstParamId);
