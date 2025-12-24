use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A single component of a path.
///
/// A [`Utf8Component`] roughly corresponds to a substring between path separators
/// (`/` or `\`).
///
/// This `enum` is created by iterating over [`Utf8Components`], which in turn is
/// created by the [`components`](Utf8Path::components) method on [`Utf8Path`].
///
/// # Examples
///
/// ```rust
/// use camino::{Utf8Component, Utf8Path};
///
/// let path = Utf8Path::new("/tmp/foo/bar.txt");
/// let components = path.components().collect::<Vec<_>>();
/// assert_eq!(&components, &[
///     Utf8Component::RootDir,
///     Utf8Component::Normal("tmp"),
///     Utf8Component::Normal("foo"),
///     Utf8Component::Normal("bar.txt"),
/// ]);
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Utf8Component<'a> {
    /// A Windows path prefix, e.g., `C:` or `\\server\share`.
    ///
    /// There is a large variety of prefix types, see [`Utf8Prefix`]'s documentation
    /// for more.
    ///
    /// Does not occur on Unix.
    Prefix(Utf8PrefixComponent<'a>),
    /// The root directory component, appears after any prefix and before anything else.
    ///
    /// It represents a separator that designates that a path starts from root.
    RootDir,
    /// A reference to the current directory, i.e., `.`.
    CurDir,
    /// A reference to the parent directory, i.e., `..`.
    ParentDir,
    /// A normal component, e.g., `a` and `b` in `a/b`.
    ///
    /// This variant is the most common one, it represents references to files
    /// or directories.
    Normal(&'a str),
}
