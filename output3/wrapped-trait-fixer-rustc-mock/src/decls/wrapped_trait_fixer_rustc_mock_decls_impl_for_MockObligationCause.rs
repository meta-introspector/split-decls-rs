use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl MockObligationCause {
    pub fn new(_span: Span, _body_id: LocalDefId, _code: MockObligationCauseCode) -> Self {
        MockObligationCause
    }
}
