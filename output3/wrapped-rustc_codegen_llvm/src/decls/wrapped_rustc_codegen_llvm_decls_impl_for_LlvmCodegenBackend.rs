use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl LlvmCodegenBackend {
    pub fn new() -> Box<dyn CodegenBackend> {
        Box::new(LlvmCodegenBackend(()))
    }
}
