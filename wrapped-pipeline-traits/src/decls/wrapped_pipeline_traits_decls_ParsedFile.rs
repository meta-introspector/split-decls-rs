use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Clone)]
pub struct ParsedFile(pub String, pub PathBuf);
