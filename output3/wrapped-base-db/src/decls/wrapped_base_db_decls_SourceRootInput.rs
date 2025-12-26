use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[salsa_macros::input(debug)]
pub struct SourceRootInput {
    pub source_root: Arc<SourceRoot>,
}
