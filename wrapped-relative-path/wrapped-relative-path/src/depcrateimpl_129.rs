// Generated macro for impl_129 (impl)
macro_rules! Depcrateimpl_129 {
() => {
// Module: crate
// Provides: {"impl_129"}
// Dependencies: {}
# [cfg (feature = "std")] # [cfg_attr (relative_path_docsrs , doc (cfg (feature = "std")))] impl fmt :: Display for FromPathError { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { match self . kind { FromPathErrorKind :: NonRelative => "path contains non-relative component" . fmt (fmt) , FromPathErrorKind :: NonUtf8 => "path contains non-utf8 component" . fmt (fmt) , FromPathErrorKind :: BadSeparator => { "path contains platform-specific path separator" . fmt (fmt) } } } }
};
}
