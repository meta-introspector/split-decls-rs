// Generated macro for impl_220 (impl)
macro_rules! Depcrate_collections_stringimpl_220 {
() => {
// Module: crate::collections::string
// Provides: {"impl_220"}
// Dependencies: {}
impl < 'bump > ops :: Index < ops :: RangeFull > for String < 'bump > { type Output = str ; # [inline] fn index (& self , _index : ops :: RangeFull) -> & str { unsafe { str :: from_utf8_unchecked (& self . vec) } } }
};
}
