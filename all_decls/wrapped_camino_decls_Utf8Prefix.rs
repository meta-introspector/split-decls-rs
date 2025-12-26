use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Windows path prefixes, e.g., `C:` or `\\server\share`.
///
/// Windows uses a variety of path prefix styles, including references to drive
/// volumes (like `C:`), network shared folders (like `\\server\share`), and
/// others. In addition, some path prefixes are "verbatim" (i.e., prefixed with
/// `\\?\`), in which case `/` is *not* treated as a separator and essentially
/// no normalization is performed.
///
/// # Examples
///
/// ```
/// use camino::{Utf8Component, Utf8Path, Utf8Prefix};
/// use camino::Utf8Prefix::*;
///
/// fn get_path_prefix(s: &str) -> Utf8Prefix {
///     let path = Utf8Path::new(s);
///     match path.components().next().unwrap() {
///         Utf8Component::Prefix(prefix_component) => prefix_component.kind(),
///         _ => panic!(),
///     }
/// }
///
/// # if cfg!(windows) {
/// assert_eq!(Verbatim("pictures"), get_path_prefix(r"\\?\pictures\kittens"));
/// assert_eq!(VerbatimUNC("server", "share"), get_path_prefix(r"\\?\UNC\server\share"));
/// assert_eq!(VerbatimDisk(b'C'), get_path_prefix(r"\\?\C:\"));
/// assert_eq!(DeviceNS("BrainInterface"), get_path_prefix(r"\\.\BrainInterface"));
/// assert_eq!(UNC("server", "share"), get_path_prefix(r"\\server\share"));
/// assert_eq!(Disk(b'C'), get_path_prefix(r"C:\Users\Rust\Pictures\Ferris"));
/// # }
/// ```
#[derive(Copy, Clone, Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum Utf8Prefix<'a> {
    /// Verbatim prefix, e.g., `\\?\cat_pics`.
    ///
    /// Verbatim prefixes consist of `\\?\` immediately followed by the given
    /// component.
    Verbatim(&'a str),
    /// Verbatim prefix using Windows' _**U**niform **N**aming **C**onvention_,
    /// e.g., `\\?\UNC\server\share`.
    ///
    /// Verbatim UNC prefixes consist of `\\?\UNC\` immediately followed by the
    /// server's hostname and a share name.
    VerbatimUNC(&'a str, &'a str),
    /// Verbatim disk prefix, e.g., `\\?\C:`.
    ///
    /// Verbatim disk prefixes consist of `\\?\` immediately followed by the
    /// drive letter and `:`.
    VerbatimDisk(u8),
    /// Device namespace prefix, e.g., `\\.\COM42`.
    ///
    /// Device namespace prefixes consist of `\\.\` immediately followed by the
    /// device name.
    DeviceNS(&'a str),
    /// Prefix using Windows' _**U**niform **N**aming **C**onvention_, e.g.
    /// `\\server\share`.
    ///
    /// UNC prefixes consist of the server's hostname and a share name.
    UNC(&'a str, &'a str),
    /// Prefix `C:` for the given disk drive.
    Disk(u8),
}
