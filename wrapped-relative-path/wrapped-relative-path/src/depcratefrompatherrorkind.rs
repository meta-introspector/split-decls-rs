// Generated macro for FromPathErrorKind (enum)
macro_rules! DepcrateFromPathErrorKind {
() => {
// Module: crate
// Provides: {"FromPathErrorKind"}
// Dependencies: {}
# [doc = " Error kind for [`FromPathError`]."] # [cfg (feature = "std")] # [cfg_attr (relative_path_docsrs , doc (cfg (feature = "std")))] # [derive (Debug , Clone , Copy , PartialEq , Eq)] # [non_exhaustive] pub enum FromPathErrorKind { # [doc = " Non-relative component in path."] NonRelative , # [doc = " Non-utf8 component in path."] NonUtf8 , # [doc = " Trying to convert a platform-specific path which uses a platform-specific separator."] BadSeparator , }
};
}
