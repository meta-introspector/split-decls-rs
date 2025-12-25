use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl AbsPathBuf {
    /// Wrap the given absolute path in `AbsPathBuf`
    ///
    /// # Panics
    ///
    /// Panics if `path` is not absolute.
    pub fn assert(path: Utf8PathBuf) -> AbsPathBuf {
        AbsPathBuf::try_from(path)
            .unwrap_or_else(|path| panic!("expected absolute path, got {path}"))
    }
    /// Wrap the given absolute path in `AbsPathBuf`
    ///
    /// # Panics
    ///
    /// Panics if `path` is not absolute.
    pub fn assert_utf8(path: PathBuf) -> AbsPathBuf {
        AbsPathBuf::assert(
            Utf8PathBuf::from_path_buf(path)
                .unwrap_or_else(|path| {
                    panic!("expected utf8 path, got {}", path.display())
                }),
        )
    }
    /// Coerces to an `AbsPath` slice.
    ///
    /// Equivalent of [`Utf8PathBuf::as_path`] for `AbsPathBuf`.
    pub fn as_path(&self) -> &AbsPath {
        AbsPath::assert(self.0.as_path())
    }
    /// Equivalent of [`Utf8PathBuf::pop`] for `AbsPathBuf`.
    ///
    /// Note that this won't remove the root component, so `self` will still be
    /// absolute.
    pub fn pop(&mut self) -> bool {
        self.0.pop()
    }
    /// Equivalent of [`PathBuf::push`] for `AbsPathBuf`.
    ///
    /// Extends `self` with `path`.
    ///
    /// If `path` is absolute, it replaces the current path.
    ///
    /// On Windows:
    ///
    /// * if `path` has a root but no prefix (e.g., `\windows`), it
    ///   replaces everything except for the prefix (if any) of `self`.
    /// * if `path` has a prefix but no root, it replaces `self`.
    /// * if `self` has a verbatim prefix (e.g. `\\?\C:\windows`)
    ///   and `path` is not empty, the new path is normalized: all references
    ///   to `.` and `..` are removed.
    pub fn push<P: AsRef<Utf8Path>>(&mut self, suffix: P) {
        self.0.push(suffix)
    }
    pub fn join(&self, path: impl AsRef<Utf8Path>) -> Self {
        Self(self.0.join(path))
    }
}
