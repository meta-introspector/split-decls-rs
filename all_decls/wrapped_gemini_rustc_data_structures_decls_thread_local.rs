use serde::{Deserialize, Serialize};
use std::collections::HashMap;
thread_local! {
    pub static COLLECTED_DIAGNOSTICS : Lazy < RwLock < Vec < SerializableDiagnostic >>> =
    Lazy::new(|| RwLock::new(Vec::new()));
}
