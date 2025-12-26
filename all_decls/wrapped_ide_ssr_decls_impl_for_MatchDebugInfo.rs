use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(test)]
impl MatchDebugInfo {
    pub fn match_failure_reason(&self) -> Option<&str> {
        self.matched.as_ref().err().map(|r| r.reason.as_str())
    }
}
