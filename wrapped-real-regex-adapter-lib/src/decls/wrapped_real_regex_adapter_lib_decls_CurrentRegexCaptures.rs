use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(feature = "regex_enabled"))]
pub type CurrentRegexCaptures<'t> = DummyRegexCaptures;
