use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Command-line arguments
#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct RawArgs {
    items: Vec<OsString>,
}
