use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl From<String> for Utf8PathBuf {
    fn from(string: String) -> Utf8PathBuf {
        Utf8PathBuf(string.into())
    }
}
