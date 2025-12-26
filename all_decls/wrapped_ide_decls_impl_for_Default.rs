use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Default for AnalysisHost {
    fn default() -> AnalysisHost {
        AnalysisHost::new(None)
    }
}
