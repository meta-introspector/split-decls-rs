// Generated macro for impl_127 (impl)
macro_rules! Depcrateimpl_127 {
() => {
// Module: crate
// Provides: {"impl_127"}
// Dependencies: {}
# [cfg (feature = "std")] # [cfg_attr (relative_path_docsrs , doc (cfg (feature = "std")))] impl FromPathError { # [doc = " Gets the underlying [`FromPathErrorKind`] that provides more details on"] # [doc = " what went wrong."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::path::Path;"] # [doc = " use relative_path::{FromPathErrorKind, RelativePathBuf};"] # [doc = ""] # [doc = " let result = RelativePathBuf::from_path(Path::new(\"/hello/world\"));"] # [doc = " let e = result.unwrap_err();"] # [doc = ""] # [doc = " assert_eq!(FromPathErrorKind::NonRelative, e.kind());"] # [doc = " ```"] # [must_use] # [inline] pub fn kind (& self) -> FromPathErrorKind { self . kind } }
};
}
