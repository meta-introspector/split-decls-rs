// Generated macro for impl_226 (impl)
macro_rules! Depcrate_collections_stringimpl_226 {
() => {
// Module: crate::collections::string
// Provides: {"impl_226"}
// Dependencies: {}
impl < 'bump > ops :: IndexMut < ops :: RangeFull > for String < 'bump > { # [inline] fn index_mut (& mut self , _index : ops :: RangeFull) -> & mut str { unsafe { str :: from_utf8_unchecked_mut (& mut * self . vec) } } }
};
}
