// Generated macro for impl_312 (impl)
macro_rules! Depcrate_rust_typeimpl_312 {
() => {
// Module: crate::rust_type
// Provides: {"impl_312"}
// Dependencies: {}
impl ParsePosition { fn strip < 'a > (self , s : & 'a str , needle : & str) -> Option < & 'a str > { match self { Self :: Suffix => s . strip_suffix (needle) , Self :: Prefix => s . strip_prefix (needle) , } } }
};
}
