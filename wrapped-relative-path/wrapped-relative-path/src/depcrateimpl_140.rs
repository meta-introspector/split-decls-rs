// Generated macro for impl_140 (impl)
macro_rules! Depcrateimpl_140 {
() => {
// Module: crate
// Provides: {"impl_140"}
// Dependencies: {}
# [doc = " Conversion from [`RelativePathBuf`] to [`Arc<RelativePath>`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::sync::Arc;"] # [doc = " use relative_path::{RelativePath, RelativePathBuf};"] # [doc = ""] # [doc = " let path = RelativePathBuf::from(\"foo/bar\");"] # [doc = " let path: Arc<RelativePath> = path.into();"] # [doc = " assert_eq!(&*path, \"foo/bar\");"] # [doc = " ```"] # [cfg (feature = "alloc")] impl From < RelativePathBuf > for Arc < RelativePath > { # [inline] fn from (path : RelativePathBuf) -> Arc < RelativePath > { let arc = Arc :: < str > :: from (path . into_string ()) ; let rw = Arc :: into_raw (arc) as * const RelativePath ; unsafe { Arc :: from_raw (rw) } } }
};
}
