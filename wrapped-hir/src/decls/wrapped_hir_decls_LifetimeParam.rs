use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LifetimeParam {
    pub(crate) id: LifetimeParamId,
}
