// Generated macro for impl_108 (impl)
macro_rules! Depcrate_parseimpl_108 {
() => {
// Module: crate::parse
// Provides: {"impl_108"}
// Dependencies: {}
# [doc = " Instantiation"] impl < 'a > Lines < 'a > { # [doc = " Create a new instance to parse all attributes in all lines of the input `bytes`."] pub fn new (bytes : & 'a [u8]) -> Self { let bom = unicode_bom :: Bom :: from (bytes) ; Lines { lines : bytes [bom . len () ..] . lines () , line_no : 0 , } } }
};
}
