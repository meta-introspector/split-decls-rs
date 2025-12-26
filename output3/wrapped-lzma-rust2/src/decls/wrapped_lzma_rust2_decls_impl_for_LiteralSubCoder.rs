use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl LiteralSubCoder {
    pub fn new() -> Self {
        let probs = [PROB_INIT; 0x300];
        Self { probs }
    }
    pub fn reset(&mut self) {
        self.probs = [PROB_INIT; 0x300];
    }
}
