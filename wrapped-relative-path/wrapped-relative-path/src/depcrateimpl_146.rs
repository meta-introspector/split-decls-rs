// Generated macro for impl_146 (impl)
macro_rules! Depcrateimpl_146 {
() => {
// Module: crate
// Provides: {"impl_146"}
// Dependencies: {}
# [doc = " [`AsRef<RelativePath>`] implementation for [`str`]."] # [doc = ""] # [doc = " [`str`]: prim@str"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use relative_path::RelativePath;"] # [doc = ""] # [doc = " let path: &RelativePath = \"foo/bar\".as_ref();"] # [doc = " assert_eq!(path, RelativePath::new(\"foo/bar\"));"] # [doc = " ```"] impl AsRef < RelativePath > for str { # [inline] fn as_ref (& self) -> & RelativePath { RelativePath :: new (self) } }
};
}
