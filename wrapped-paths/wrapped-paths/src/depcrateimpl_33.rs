// Generated macro for impl_33 (impl)
macro_rules! Depcrateimpl_33 {
() => {
// Module: crate
// Provides: {"impl_33"}
// Dependencies: {}
impl RelPathBuf { # [doc = " Coerces to a `RelPath` slice."] # [doc = ""] # [doc = " Equivalent of [`Utf8PathBuf::as_path`] for `RelPathBuf`."] pub fn as_path (& self) -> & RelPath { RelPath :: new_unchecked (self . 0 . as_path ()) } }
};
}
