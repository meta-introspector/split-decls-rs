use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl DummyRegexMatcher {
    pub fn new(_re: &str) -> Result<Self> {
        Ok(DummyRegexMatcher)
    }
}
