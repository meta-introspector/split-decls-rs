use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl MockInferCtxtBuilder {
    pub fn build(self, _typing_mode: MockTypingMode) -> MockInferCtxt {
        MockInferCtxt
    }
}
