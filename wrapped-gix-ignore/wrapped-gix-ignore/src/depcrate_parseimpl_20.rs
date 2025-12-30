// Generated macro for impl_20 (impl)
macro_rules! Depcrate_parseimpl_20 {
() => {
// Module: crate::parse
// Provides: {"impl_20"}
// Dependencies: {}
impl < 'a > Lines < 'a > { # [doc = " Create a new instance from `buf` to parse ignore patterns from."] # [doc = ""] # [doc = " If `support_precious` is `true`, we will parse `$` prefixed entries as precious."] # [doc = " This is backward-incompatible as files that actually start with `$` like `$houdini`"] # [doc = " will then not be ignored anymore, instead it ignores `houdini`."] pub fn new (buf : & 'a [u8] , support_precious : bool) -> Self { let bom = unicode_bom :: Bom :: from (buf) ; Lines { lines : buf [bom . len () ..] . lines () , line_no : 0 , support_precious , } } }
};
}
