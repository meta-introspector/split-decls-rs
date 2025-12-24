use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl AsRef<Utf8Path> for RelPath {
    fn as_ref(&self) -> &Utf8Path {
        &self.0
    }
}
