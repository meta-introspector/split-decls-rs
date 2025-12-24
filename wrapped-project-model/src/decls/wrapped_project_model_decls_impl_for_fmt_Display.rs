use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl fmt::Display for ProjectManifest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.manifest_path(), f)
    }
}
