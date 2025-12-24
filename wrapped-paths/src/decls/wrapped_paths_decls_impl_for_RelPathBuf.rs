use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl RelPathBuf {
    /// Coerces to a `RelPath` slice.
    ///
    /// Equivalent of [`Utf8PathBuf::as_path`] for `RelPathBuf`.
    pub fn as_path(&self) -> &RelPath {
        RelPath::new_unchecked(self.0.as_path())
    }
}
