use serde::{Deserialize, Serialize};
use std::collections::HashMap;
static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
