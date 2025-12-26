use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An iterator over [`Utf8Path`] and its ancestors.
///
/// This `struct` is created by the [`ancestors`] method on [`Utf8Path`].
/// See its documentation for more.
///
/// # Examples
///
/// ```
/// use camino::Utf8Path;
///
/// let path = Utf8Path::new("/foo/bar");
///
/// for ancestor in path.ancestors() {
///     println!("{}", ancestor);
/// }
/// ```
///
/// [`ancestors`]: Utf8Path::ancestors
#[derive(Copy, Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[repr(transparent)]
pub struct Utf8Ancestors<'a>(Ancestors<'a>);
