// Generated macro for FromPathError (struct)
macro_rules! DepcrateFromPathError {
() => {
// Module: crate
// Provides: {"FromPathError"}
// Dependencies: {}
# [doc = " An error raised when attempting to convert a path using"] # [doc = " [`RelativePathBuf::from_path`]."] # [cfg (feature = "std")] # [cfg_attr (relative_path_docsrs , doc (cfg (feature = "std")))] # [derive (Debug , Clone , PartialEq , Eq)] pub struct FromPathError { kind : FromPathErrorKind , }
};
}
