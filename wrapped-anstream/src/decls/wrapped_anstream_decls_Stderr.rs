use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An adaptive wrapper around the global standard error stream of the current process
pub type Stderr = AutoStream<std::io::Stderr>;
