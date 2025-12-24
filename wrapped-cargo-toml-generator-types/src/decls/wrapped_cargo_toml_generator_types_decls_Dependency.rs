use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone, PartialEq)]
pub enum Dependency {
    Version(String),
    Table(DependencyTable),
}
