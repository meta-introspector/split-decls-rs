use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum InvocationStrategy {
    Once,
    #[default]
    PerWorkspace,
}
