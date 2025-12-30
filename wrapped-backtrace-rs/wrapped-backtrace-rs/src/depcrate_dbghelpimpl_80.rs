// Generated macro for impl_80 (impl)
macro_rules! Depcrate_dbghelpimpl_80 {
() => {
// Module: crate::dbghelp
// Provides: {"impl_80"}
// Dependencies: {}
impl SearchPath { fn new (initial_search_path : Vec < u16 >) -> Self { Self { search_path_utf16 : initial_search_path , } } # [doc = " Add a path to the search path if it is not already present."] fn add (& mut self , path : & [u16]) { let sep = utf16_char (';') ; if ! self . search_path_utf16 . split (| & c | c == sep) . any (| p | p == path) { if self . search_path_utf16 . last () != Some (& sep) { self . search_path_utf16 . push (sep) ; } self . search_path_utf16 . extend_from_slice (path) ; } } fn finalize (mut self) -> Vec < u16 > { self . search_path_utf16 . push (0) ; self . search_path_utf16 } }
};
}
