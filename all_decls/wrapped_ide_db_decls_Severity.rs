use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Severity {
    Error,
    Warning,
    WeakWarning,
    Allow,
}
