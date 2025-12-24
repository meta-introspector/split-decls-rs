use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Utf8Prefix<'_> {
    /// Determines if the prefix is verbatim, i.e., begins with `\\?\`.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Prefix::*;
    ///
    /// assert!(Verbatim("pictures").is_verbatim());
    /// assert!(VerbatimUNC("server", "share").is_verbatim());
    /// assert!(VerbatimDisk(b'C').is_verbatim());
    /// assert!(!DeviceNS("BrainInterface").is_verbatim());
    /// assert!(!UNC("server", "share").is_verbatim());
    /// assert!(!Disk(b'C').is_verbatim());
    /// ```
    #[must_use]
    pub fn is_verbatim(&self) -> bool {
        use Utf8Prefix::*;
        matches!(self, Verbatim(_) | VerbatimDisk(_) | VerbatimUNC(..))
    }
}
