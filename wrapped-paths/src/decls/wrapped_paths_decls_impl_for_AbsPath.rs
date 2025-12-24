use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl AbsPath {
    /// Wrap the given absolute path in `AbsPath`
    ///
    /// # Panics
    ///
    /// Panics if `path` is not absolute.
    pub fn assert(path: &Utf8Path) -> &AbsPath {
        assert!(path.is_absolute(), "{path} is not absolute");
        unsafe { &*(path as *const Utf8Path as *const AbsPath) }
    }
    /// Equivalent of [`Utf8Path::parent`] for `AbsPath`.
    pub fn parent(&self) -> Option<&AbsPath> {
        self.0.parent().map(AbsPath::assert)
    }
    /// Equivalent of [`Utf8Path::join`] for `AbsPath` with an additional normalize step afterwards.
    pub fn absolutize(&self, path: impl AsRef<Utf8Path>) -> AbsPathBuf {
        self.join(path).normalize()
    }
    /// Equivalent of [`Utf8Path::join`] for `AbsPath`.
    pub fn join(&self, path: impl AsRef<Utf8Path>) -> AbsPathBuf {
        Utf8Path::join(self.as_ref(), path).try_into().unwrap()
    }
    /// Normalize the given path:
    /// - Removes repeated separators: `/a//b` becomes `/a/b`
    /// - Removes occurrences of `.` and resolves `..`.
    /// - Removes trailing slashes: `/a/b/` becomes `/a/b`.
    ///
    /// # Example
    /// ```ignore
    /// # use paths::AbsPathBuf;
    /// let abs_path_buf = AbsPathBuf::assert("/a/../../b/.//c//".into());
    /// let normalized = abs_path_buf.normalize();
    /// assert_eq!(normalized, AbsPathBuf::assert("/b/c".into()));
    /// ```
    pub fn normalize(&self) -> AbsPathBuf {
        AbsPathBuf(normalize_path(&self.0))
    }
    /// Equivalent of [`Utf8Path::to_path_buf`] for `AbsPath`.
    pub fn to_path_buf(&self) -> AbsPathBuf {
        AbsPathBuf::try_from(self.0.to_path_buf()).unwrap()
    }
    pub fn canonicalize(&self) -> ! {
        panic!(
            "We explicitly do not provide canonicalization API, as that is almost always a wrong solution, see #14430"
        )
    }
    /// Equivalent of [`Utf8Path::strip_prefix`] for `AbsPath`.
    ///
    /// Returns a relative path.
    pub fn strip_prefix(&self, base: &AbsPath) -> Option<&RelPath> {
        self.0.strip_prefix(base).ok().map(RelPath::new_unchecked)
    }
    pub fn starts_with(&self, base: &AbsPath) -> bool {
        self.0.starts_with(&base.0)
    }
    pub fn ends_with(&self, suffix: &RelPath) -> bool {
        self.0.ends_with(&suffix.0)
    }
    pub fn name_and_extension(&self) -> Option<(&str, Option<&str>)> {
        Some((self.file_stem()?, self.extension()))
    }
    pub fn file_name(&self) -> Option<&str> {
        self.0.file_name()
    }
    pub fn extension(&self) -> Option<&str> {
        self.0.extension()
    }
    pub fn file_stem(&self) -> Option<&str> {
        self.0.file_stem()
    }
    pub fn as_os_str(&self) -> &OsStr {
        self.0.as_os_str()
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
    #[deprecated(note = "use Display instead")]
    pub fn display(&self) -> ! {
        unimplemented!()
    }
    #[deprecated(note = "use std::fs::metadata().is_ok() instead")]
    pub fn exists(&self) -> ! {
        unimplemented!()
    }
    pub fn components(&self) -> Utf8Components<'_> {
        self.0.components()
    }
}
