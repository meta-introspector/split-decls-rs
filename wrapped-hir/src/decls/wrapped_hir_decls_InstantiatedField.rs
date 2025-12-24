use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InstantiatedField<'db> {
    pub(crate) inner: Field,
    pub(crate) args: GenericArgs<'db>,
}
