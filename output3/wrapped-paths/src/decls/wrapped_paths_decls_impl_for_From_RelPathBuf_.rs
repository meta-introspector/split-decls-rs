use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl From<RelPathBuf> for Utf8PathBuf {
    fn from(RelPathBuf(path_buf): RelPathBuf) -> Utf8PathBuf {
        path_buf
    }
}
