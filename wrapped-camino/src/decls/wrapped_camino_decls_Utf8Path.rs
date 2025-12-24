use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A slice of a UTF-8 path (akin to [`str`]).
///
/// This type supports a number of operations for inspecting a path, including
/// breaking the path into its components (separated by `/` on Unix and by either
/// `/` or `\` on Windows), extracting the file name, determining whether the path
/// is absolute, and so on.
///
/// This is an *unsized* type, meaning that it must always be used behind a
/// pointer like `&` or [`Box`]. For an owned version of this type,
/// see [`Utf8PathBuf`].
///
/// # Examples
///
/// ```
/// use camino::Utf8Path;
///
/// // Note: this example does work on Windows
/// let path = Utf8Path::new("./foo/bar.txt");
///
/// let parent = path.parent();
/// assert_eq!(parent, Some(Utf8Path::new("./foo")));
///
/// let file_stem = path.file_stem();
/// assert_eq!(file_stem, Some("bar"));
///
/// let extension = path.extension();
/// assert_eq!(extension, Some("txt"));
/// ```
#[repr(transparent)]
pub struct Utf8Path(Path);
