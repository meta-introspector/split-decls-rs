use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl TestEnvCommandExt for snapbox::cmd::Command {
    fn current_dir<S: AsRef<std::path::Path>>(self, path: S) -> Self {
        self.current_dir(path)
    }
    fn env<S: AsRef<std::ffi::OsStr>>(self, key: &str, value: S) -> Self {
        self.env(key, value)
    }
    fn env_remove(self, key: &str) -> Self {
        self.env_remove(key)
    }
}
