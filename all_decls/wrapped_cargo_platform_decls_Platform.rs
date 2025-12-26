use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Platform definition.
#[derive(Eq, PartialEq, Hash, Ord, PartialOrd, Clone, Debug)]
pub enum Platform {
    /// A named platform, like `x86_64-apple-darwin`.
    Name(String),
    /// A cfg expression, like `cfg(windows)`.
    Cfg(CfgExpr),
}
