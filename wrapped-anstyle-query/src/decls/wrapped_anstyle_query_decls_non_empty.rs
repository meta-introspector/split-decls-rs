use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn non_empty(var: Option<&std::ffi::OsStr>) -> bool {
    !var.unwrap_or_default().is_empty()
}
