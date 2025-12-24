use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Makes the path absolute without accessing the filesystem, converting it to a [`Utf8PathBuf`].
///
/// If the path is relative, the current directory is used as the base directory. All intermediate
/// components will be resolved according to platform-specific rules, but unlike
/// [`canonicalize`][Utf8Path::canonicalize] or [`canonicalize_utf8`](Utf8Path::canonicalize_utf8),
/// this does not resolve symlinks and may succeed even if the path does not exist.
///
/// *Requires Rust 1.79 or newer.*
///
/// # Errors
///
/// Errors if:
///
/// * The path is empty.
/// * The [current directory][std::env::current_dir] cannot be determined.
/// * The path is not valid UTF-8.
///
/// # Examples
///
/// ## POSIX paths
///
/// ```
/// # #[cfg(unix)]
/// fn main() -> std::io::Result<()> {
///     use camino::Utf8Path;
///
///     // Relative to absolute
///     let absolute = camino::absolute_utf8("foo/./bar")?;
///     assert!(absolute.ends_with("foo/bar"));
///
///     // Absolute to absolute
///     let absolute = camino::absolute_utf8("/foo//test/.././bar.rs")?;
///     assert_eq!(absolute, Utf8Path::new("/foo/test/../bar.rs"));
///     Ok(())
/// }
/// # #[cfg(not(unix))]
/// # fn main() {}
/// ```
///
/// The path is resolved using [POSIX semantics][posix-semantics] except that it stops short of
/// resolving symlinks. This means it will keep `..` components and trailing slashes.
///
/// ## Windows paths
///
/// ```
/// # #[cfg(windows)]
/// fn main() -> std::io::Result<()> {
///     use camino::Utf8Path;
///
///     // Relative to absolute
///     let absolute = camino::absolute_utf8("foo/./bar")?;
///     assert!(absolute.ends_with(r"foo\bar"));
///
///     // Absolute to absolute
///     let absolute = camino::absolute_utf8(r"C:\foo//test\..\./bar.rs")?;
///
///     assert_eq!(absolute, Utf8Path::new(r"C:\foo\bar.rs"));
///     Ok(())
/// }
/// # #[cfg(not(windows))]
/// # fn main() {}
/// ```
///
/// For verbatim paths this will simply return the path as given. For other paths this is currently
/// equivalent to calling [`GetFullPathNameW`][windows-path].
///
/// Note that this [may change in the future][changes].
///
/// [changes]: io#platform-specific-behavior
/// [posix-semantics]:
///     https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap04.html#tag_04_13
/// [windows-path]:
///     https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfullpathnamew
#[cfg(absolute_path)]
pub fn absolute_utf8<P: AsRef<Path>>(path: P) -> io::Result<Utf8PathBuf> {
    let path = path.as_ref();
    #[allow(clippy::incompatible_msrv)]
    Utf8PathBuf::try_from(std::path::absolute(path)?)
        .map_err(|error| error.into_io_error())
}
