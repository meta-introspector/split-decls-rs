// Generated macro for impl_136 (impl)
macro_rules! Depcrateimpl_136 {
() => {
// Module: crate
// Provides: {"impl_136"}
// Dependencies: {}
# [doc = " Conversion from a [`str`] reference to a [`Box<RelativePath>`]."] # [doc = ""] # [doc = " [`str`]: prim@str"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use relative_path::RelativePath;"] # [doc = ""] # [doc = " let path: Box<RelativePath> = \"foo/bar\".into();"] # [doc = " assert_eq!(&*path, \"foo/bar\");"] # [doc = ""] # [doc = " let path: Box<RelativePath> = RelativePath::new(\"foo/bar\").into();"] # [doc = " assert_eq!(&*path, \"foo/bar\");"] # [doc = " ```"] # [cfg (feature = "alloc")] impl < T > From < & T > for Box < RelativePath > where T : ? Sized + AsRef < str > , { # [inline] fn from (path : & T) -> Box < RelativePath > { Box :: < RelativePath > :: from (Box :: < str > :: from (path . as_ref ())) } }
};
}
