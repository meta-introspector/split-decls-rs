// Generated macro for impl_139 (impl)
macro_rules! Depcrateimpl_139 {
() => {
// Module: crate
// Provides: {"impl_139"}
// Dependencies: {}
# [doc = " Conversion from [`RelativePath`] to [`Arc<RelativePath>`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::sync::Arc;"] # [doc = " use relative_path::RelativePath;"] # [doc = ""] # [doc = " let path: Arc<RelativePath> = RelativePath::new(\"foo/bar\").into();"] # [doc = " assert_eq!(&*path, \"foo/bar\");"] # [doc = " ```"] # [cfg (feature = "alloc")] impl From < & RelativePath > for Arc < RelativePath > { # [inline] fn from (path : & RelativePath) -> Arc < RelativePath > { let arc : Arc < str > = path . inner . into () ; let rw = Arc :: into_raw (arc) as * const RelativePath ; unsafe { Arc :: from_raw (rw) } } }
};
}
