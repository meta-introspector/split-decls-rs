use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl ArgCursor {
    fn new() -> Self {
        Self { cursor: 0 }
    }
}
