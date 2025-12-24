use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone)]
pub struct ValidatedFile(pub String, pub PathBuf);
