use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[path = "wrapper.rs"]
#[cfg(wrap_proc_macro)]
mod imp;
