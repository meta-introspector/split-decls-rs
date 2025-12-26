use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub fn add_diagnostic(diag: SerializableDiagnostic) {
    COLLECTED_DIAGNOSTICS.with(|diagnostics| {
        diagnostics.write().push(diag);
    });
}
