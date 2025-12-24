use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<'a> Utf8Components<'a> {
    /// Extracts a slice corresponding to the portion of the path remaining for iteration.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// let mut components = Utf8Path::new("/tmp/foo/bar.txt").components();
    /// components.next();
    /// components.next();
    ///
    /// assert_eq!(Utf8Path::new("foo/bar.txt"), components.as_path());
    /// ```
    #[must_use]
    #[inline]
    pub fn as_path(&self) -> &'a Utf8Path {
        unsafe { Utf8Path::assume_utf8(self.0.as_path()) }
    }
}
