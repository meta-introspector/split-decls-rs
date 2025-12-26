use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl MockInferCtxt {
    pub fn probe(self, f: impl FnOnce(&MockInferCtxt) -> bool) -> bool {
        f(&self)
    }
    pub fn at(self, _cause: &MockObligationCause, _param_env: MockParamEnv) -> MockInferCtxtAt {
        MockInferCtxtAt
    }
}
