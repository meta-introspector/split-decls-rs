// Generated macro for impl_135 (impl)
macro_rules! Depcrateimpl_135 {
() => {
// Module: crate
// Provides: {"impl_135"}
// Dependencies: {}
# [doc = " Conversion from a [`Box<str>`] reference to a [`Box<RelativePath>`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use relative_path::RelativePath;"] # [doc = ""] # [doc = " let path: Box<RelativePath> = Box::<str>::from(\"foo/bar\").into();"] # [doc = " assert_eq!(&*path, \"foo/bar\");"] # [doc = " ```"] # [cfg (feature = "alloc")] impl From < Box < str > > for Box < RelativePath > { # [inline] fn from (boxed : Box < str >) -> Box < RelativePath > { let rw = Box :: into_raw (boxed) as * mut RelativePath ; unsafe { Box :: from_raw (rw) } } }
};
}
