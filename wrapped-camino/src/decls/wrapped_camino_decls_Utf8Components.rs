use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An iterator over the [`Utf8Component`]s of a [`Utf8Path`].
///
/// This `struct` is created by the [`components`] method on [`Utf8Path`].
/// See its documentation for more.
///
/// # Examples
///
/// ```
/// use camino::Utf8Path;
///
/// let path = Utf8Path::new("/tmp/foo/bar.txt");
///
/// for component in path.components() {
///     println!("{:?}", component);
/// }
/// ```
///
/// [`components`]: Utf8Path::components
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Utf8Components<'a>(Components<'a>);
