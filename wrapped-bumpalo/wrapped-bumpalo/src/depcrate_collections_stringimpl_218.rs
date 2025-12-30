// Generated macro for impl_218 (impl)
macro_rules! Depcrate_collections_stringimpl_218 {
() => {
// Module: crate::collections::string
// Provides: {"impl_218"}
// Dependencies: {}
impl < 'bump > ops :: Index < ops :: RangeTo < usize > > for String < 'bump > { type Output = str ; # [inline] fn index (& self , index : ops :: RangeTo < usize >) -> & str { & self [..] [index] } }
};
}
