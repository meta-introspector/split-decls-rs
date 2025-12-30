// Generated macro for impl_143 (impl)
macro_rules! Depcrateimpl_143 {
() => {
// Module: crate
// Provides: {"impl_143"}
// Dependencies: {}
# [doc = " [`ToOwned`] implementation for [`RelativePath`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use relative_path::RelativePath;"] # [doc = ""] # [doc = " let path = RelativePath::new(\"foo/bar\").to_owned();"] # [doc = " assert_eq!(path, \"foo/bar\");"] # [doc = " ```"] # [cfg (feature = "alloc")] impl ToOwned for RelativePath { type Owned = RelativePathBuf ; # [inline] fn to_owned (& self) -> RelativePathBuf { self . to_relative_path_buf () } }
};
}
