use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub fn clear_diagnostics() {
    COLLECTED_DIAGNOSTICS.with(|diagnostics| {
        diagnostics.write().clear();
    });
}
