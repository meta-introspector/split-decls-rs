use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone)]
pub struct PanicMessage {
    message: Option<String>,
}
