// Generated macro for impl_120 (impl)
macro_rules! Depcrateimpl_120 {
() => {
// Module: crate
// Provides: {"impl_120"}
// Dependencies: {}
impl < 'a > Components < 'a > { # [doc = " Construct a new component from the given string."] # [inline] fn new (source : & 'a str) -> Components < 'a > { Self { source } } # [doc = " Extracts a slice corresponding to the portion of the path remaining for iteration."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use relative_path::RelativePath;"] # [doc = ""] # [doc = " let mut components = RelativePath::new(\"tmp/foo/bar.txt\").components();"] # [doc = " components.next();"] # [doc = " components.next();"] # [doc = ""] # [doc = " assert_eq!(\"bar.txt\", components.as_relative_path());"] # [doc = " ```"] # [must_use] # [inline] pub fn as_relative_path (& self) -> & 'a RelativePath { RelativePath :: new (self . source) } }
};
}
