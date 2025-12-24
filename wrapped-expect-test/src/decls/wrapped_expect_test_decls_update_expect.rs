use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn update_expect() -> bool {
    env::var("UPDATE_EXPECT").is_ok()
}
