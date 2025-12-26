use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A structure wrapping a Windows path prefix as well as its unparsed string
/// representation.
///
/// In addition to the parsed [`Utf8Prefix`] information returned by [`kind`],
/// [`Utf8PrefixComponent`] also holds the raw and unparsed [`str`] slice,
/// returned by [`as_str`].
///
/// Instances of this `struct` can be obtained by matching against the
/// [`Prefix` variant] on [`Utf8Component`].
///
/// Does not occur on Unix.
///
/// # Examples
///
/// ```
/// # if cfg!(windows) {
/// use camino::{Utf8Component, Utf8Path, Utf8Prefix};
/// use std::ffi::OsStr;
///
/// let path = Utf8Path::new(r"C:\you\later\");
/// match path.components().next().unwrap() {
///     Utf8Component::Prefix(prefix_component) => {
///         assert_eq!(Utf8Prefix::Disk(b'C'), prefix_component.kind());
///         assert_eq!("C:", prefix_component.as_str());
///     }
///     _ => unreachable!(),
/// }
/// # }
/// ```
///
/// [`as_str`]: Utf8PrefixComponent::as_str
/// [`kind`]: Utf8PrefixComponent::kind
/// [`Prefix` variant]: Utf8Component::Prefix
#[repr(transparent)]
#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Utf8PrefixComponent<'a>(PrefixComponent<'a>);
