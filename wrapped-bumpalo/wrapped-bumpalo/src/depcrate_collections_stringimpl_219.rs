// Generated macro for impl_219 (impl)
macro_rules! Depcrate_collections_stringimpl_219 {
() => {
// Module: crate::collections::string
// Provides: {"impl_219"}
// Dependencies: {}
impl < 'bump > ops :: Index < ops :: RangeFrom < usize > > for String < 'bump > { type Output = str ; # [inline] fn index (& self , index : ops :: RangeFrom < usize >) -> & str { & self [..] [index] } }
};
}
