// Generated macro for impl_141 (impl)
macro_rules! Depcrateimpl_141 {
() => {
// Module: crate
// Provides: {"impl_141"}
// Dependencies: {}
# [doc = " Conversion from [`RelativePathBuf`] to [`Rc<RelativePath>`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::rc::Rc;"] # [doc = " use relative_path::RelativePath;"] # [doc = ""] # [doc = " let path: Rc<RelativePath> = RelativePath::new(\"foo/bar\").into();"] # [doc = " assert_eq!(&*path, \"foo/bar\");"] # [doc = " ```"] # [cfg (feature = "alloc")] impl From < & RelativePath > for Rc < RelativePath > { # [inline] fn from (path : & RelativePath) -> Rc < RelativePath > { let rc : Rc < str > = path . inner . into () ; let rw = Rc :: into_raw (rc) as * const RelativePath ; unsafe { Rc :: from_raw (rw) } } }
};
}
