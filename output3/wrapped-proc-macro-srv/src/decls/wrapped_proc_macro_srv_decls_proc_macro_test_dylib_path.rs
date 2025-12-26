use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(test)]
pub fn proc_macro_test_dylib_path() -> paths::Utf8PathBuf {
    proc_macro_test::PROC_MACRO_TEST_LOCATION.into()
}
