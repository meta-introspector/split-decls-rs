use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(windows)]
pub fn path_to_c_string(p: &Path) -> CString {
    CString::new(p.to_str().unwrap()).unwrap()
}
