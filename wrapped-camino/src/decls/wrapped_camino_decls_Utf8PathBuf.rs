use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An owned, mutable UTF-8 path (akin to [`String`]).
///
/// This type provides methods like [`push`] and [`set_extension`] that mutate
/// the path in place. It also implements [`Deref`] to [`Utf8Path`], meaning that
/// all methods on [`Utf8Path`] slices are available on [`Utf8PathBuf`] values as well.
///
/// [`push`]: Utf8PathBuf::push
/// [`set_extension`]: Utf8PathBuf::set_extension
///
/// # Examples
///
/// You can use [`push`] to build up a [`Utf8PathBuf`] from
/// components:
///
/// ```
/// use camino::Utf8PathBuf;
///
/// let mut path = Utf8PathBuf::new();
///
/// path.push(r"C:\");
/// path.push("windows");
/// path.push("system32");
///
/// path.set_extension("dll");
/// ```
///
/// However, [`push`] is best used for dynamic situations. This is a better way
/// to do this when you know all of the components ahead of time:
///
/// ```
/// use camino::Utf8PathBuf;
///
/// let path: Utf8PathBuf = [r"C:\", "windows", "system32.dll"].iter().collect();
/// ```
///
/// We can still do better than this! Since these are all strings, we can use
/// [`From::from`]:
///
/// ```
/// use camino::Utf8PathBuf;
///
/// let path = Utf8PathBuf::from(r"C:\windows\system32.dll");
/// ```
///
/// Which method works best depends on what kind of situation you're in.
#[derive(Clone, Default)]
#[repr(transparent)]
pub struct Utf8PathBuf(PathBuf);
