use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl fmt::Debug for SourceFile {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "SourceFile({:?})", self.name)
    }
}
