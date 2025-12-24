use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn version() -> (u32, bool) {
    LazyLock::force(&VERSION).clone()
}
