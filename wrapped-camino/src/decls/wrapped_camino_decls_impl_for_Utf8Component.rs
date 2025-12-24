use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<'a> Utf8Component<'a> {
    unsafe fn new(component: Component<'a>) -> Utf8Component<'a> {
        match component {
            Component::Prefix(prefix) => {
                Utf8Component::Prefix(Utf8PrefixComponent(prefix))
            }
            Component::RootDir => Utf8Component::RootDir,
            Component::CurDir => Utf8Component::CurDir,
            Component::ParentDir => Utf8Component::ParentDir,
            Component::Normal(s) => Utf8Component::Normal(str_assume_utf8(s)),
        }
    }
    /// Extracts the underlying [`str`] slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// let path = Utf8Path::new("./tmp/foo/bar.txt");
    /// let components: Vec<_> = path.components().map(|comp| comp.as_str()).collect();
    /// assert_eq!(&components, &[".", "tmp", "foo", "bar.txt"]);
    /// ```
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &'a str {
        unsafe { str_assume_utf8(self.as_os_str()) }
    }
    /// Extracts the underlying [`OsStr`] slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// let path = Utf8Path::new("./tmp/foo/bar.txt");
    /// let components: Vec<_> = path.components().map(|comp| comp.as_os_str()).collect();
    /// assert_eq!(&components, &[".", "tmp", "foo", "bar.txt"]);
    /// ```
    #[must_use]
    pub fn as_os_str(&self) -> &'a OsStr {
        match *self {
            Utf8Component::Prefix(prefix) => prefix.as_os_str(),
            Utf8Component::RootDir => Component::RootDir.as_os_str(),
            Utf8Component::CurDir => Component::CurDir.as_os_str(),
            Utf8Component::ParentDir => Component::ParentDir.as_os_str(),
            Utf8Component::Normal(s) => OsStr::new(s),
        }
    }
}
