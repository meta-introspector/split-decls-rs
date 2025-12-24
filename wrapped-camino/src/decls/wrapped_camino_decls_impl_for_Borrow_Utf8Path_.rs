use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Borrow<Utf8Path> for Utf8PathBuf {
    #[inline]
    fn borrow(&self) -> &Utf8Path {
        self.as_path()
    }
}
