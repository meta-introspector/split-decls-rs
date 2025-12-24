use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl AsRef<Path> for RelPath {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}
