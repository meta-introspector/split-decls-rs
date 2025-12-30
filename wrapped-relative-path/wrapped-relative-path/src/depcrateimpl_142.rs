// Generated macro for impl_142 (impl)
macro_rules! Depcrateimpl_142 {
() => {
// Module: crate
// Provides: {"impl_142"}
// Dependencies: {}
# [doc = " Conversion from [`RelativePathBuf`] to [`Rc<RelativePath>`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::rc::Rc;"] # [doc = " use relative_path::{RelativePath, RelativePathBuf};"] # [doc = ""] # [doc = " let path = RelativePathBuf::from(\"foo/bar\");"] # [doc = " let path: Rc<RelativePath> = path.into();"] # [doc = " assert_eq!(&*path, \"foo/bar\");"] # [doc = " ```"] # [cfg (feature = "alloc")] impl From < RelativePathBuf > for Rc < RelativePath > { # [inline] fn from (path : RelativePathBuf) -> Rc < RelativePath > { let rc = Rc :: < str > :: from (path . into_string ()) ; let rw = Rc :: into_raw (rc) as * const RelativePath ; unsafe { Rc :: from_raw (rw) } } }
};
}
