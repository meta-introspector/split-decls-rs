use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub struct EnvSnapshot {
    vars: HashMap<OsString, OsString>,
}
