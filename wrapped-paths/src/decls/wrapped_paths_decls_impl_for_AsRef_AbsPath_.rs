use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl AsRef<AbsPath> for AbsPathBuf {
    fn as_ref(&self) -> &AbsPath {
        self.as_path()
    }
}
