// Generated macro for impl_137 (impl)
macro_rules! Depcrateimpl_137 {
() => {
// Module: crate
// Provides: {"impl_137"}
// Dependencies: {}
# [doc = " Conversion from [`RelativePathBuf`] to [`Box<RelativePath>`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::sync::Arc;"] # [doc = " use relative_path::{RelativePath, RelativePathBuf};"] # [doc = ""] # [doc = " let path = RelativePathBuf::from(\"foo/bar\");"] # [doc = " let path: Box<RelativePath> = path.into();"] # [doc = " assert_eq!(&*path, \"foo/bar\");"] # [doc = " ```"] # [cfg (feature = "alloc")] impl From < RelativePathBuf > for Box < RelativePath > { # [inline] fn from (path : RelativePathBuf) -> Box < RelativePath > { let boxed = Box :: < str > :: from (path . into_string ()) ; let rw = Box :: into_raw (boxed) as * mut RelativePath ; unsafe { Box :: from_raw (rw) } } }
};
}
