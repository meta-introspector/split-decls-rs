// Generated macro for impl_37 (impl)
macro_rules! Depcrateimpl_37 {
() => {
// Module: crate
// Provides: {"impl_37"}
// Dependencies: {}
impl RelPath { # [doc = " Creates a new `RelPath` from `path`, without checking if it is relative."] pub fn new_unchecked (path : & Utf8Path) -> & RelPath { unsafe { & * (path as * const Utf8Path as * const RelPath) } } # [doc = " Equivalent of [`Utf8Path::to_path_buf`] for `RelPath`."] pub fn to_path_buf (& self) -> RelPathBuf { RelPathBuf :: try_from (self . 0 . to_path_buf ()) . unwrap () } pub fn as_utf8_path (& self) -> & Utf8Path { self . as_ref () } pub fn as_str (& self) -> & str { self . 0 . as_str () } }
};
}
