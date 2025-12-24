use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<'a> Iter<'a> {
    /// Extracts a slice corresponding to the portion of the path remaining for iteration.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// let mut iter = Utf8Path::new("/tmp/foo/bar.txt").iter();
    /// iter.next();
    /// iter.next();
    ///
    /// assert_eq!(Utf8Path::new("foo/bar.txt"), iter.as_path());
    /// ```
    #[must_use]
    #[inline]
    pub fn as_path(&self) -> &'a Utf8Path {
        self.inner.as_path()
    }
}
