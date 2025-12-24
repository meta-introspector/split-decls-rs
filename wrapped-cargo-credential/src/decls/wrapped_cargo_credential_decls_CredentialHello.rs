use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Message sent by the credential helper on startup
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CredentialHello {
    pub v: Vec<u32>,
}
