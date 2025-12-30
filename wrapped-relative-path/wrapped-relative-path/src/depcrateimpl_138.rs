// Generated macro for impl_138 (impl)
macro_rules! Depcrateimpl_138 {
() => {
// Module: crate
// Provides: {"impl_138"}
// Dependencies: {}
# [doc = " Clone implementation for [`Box<RelativePath>`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use relative_path::RelativePath;"] # [doc = ""] # [doc = " let path: Box<RelativePath> = RelativePath::new(\"foo/bar\").into();"] # [doc = " let path2 = path.clone();"] # [doc = " assert_eq!(&*path, &*path2);"] # [doc = " ```"] # [cfg (feature = "alloc")] impl Clone for Box < RelativePath > { # [inline] fn clone (& self) -> Self { self . to_relative_path_buf () . into_boxed_relative_path () } }
};
}
