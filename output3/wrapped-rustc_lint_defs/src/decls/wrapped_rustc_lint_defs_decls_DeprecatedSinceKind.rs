use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone)]
pub enum DeprecatedSinceKind {
    InEffect,
    InFuture,
    InVersion(String),
}
