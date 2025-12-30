// Generated macro for impl_145 (impl)
macro_rules! Depcrateimpl_145 {
() => {
// Module: crate
// Provides: {"impl_145"}
// Dependencies: {}
# [doc = " [`AsRef<RelativePath>`] implementation for [String]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use relative_path::RelativePath;"] # [doc = ""] # [doc = " let path: String = format!(\"foo/bar\");"] # [doc = " let path: &RelativePath = path.as_ref();"] # [doc = " assert_eq!(path, \"foo/bar\");"] # [doc = " ```"] # [cfg (feature = "alloc")] impl AsRef < RelativePath > for String { # [inline] fn as_ref (& self) -> & RelativePath { RelativePath :: new (self) } }
};
}
