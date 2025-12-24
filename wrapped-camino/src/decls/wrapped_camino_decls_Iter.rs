use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An iterator over the [`Utf8Component`]s of a [`Utf8Path`], as [`str`] slices.
///
/// This `struct` is created by the [`iter`] method on [`Utf8Path`].
/// See its documentation for more.
///
/// [`iter`]: Utf8Path::iter
#[derive(Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Iter<'a> {
    inner: Utf8Components<'a>,
}
