// Generated macro for impl_225 (impl)
macro_rules! Depcrate_collections_stringimpl_225 {
() => {
// Module: crate::collections::string
// Provides: {"impl_225"}
// Dependencies: {}
impl < 'bump > ops :: IndexMut < ops :: RangeFrom < usize > > for String < 'bump > { # [inline] fn index_mut (& mut self , index : ops :: RangeFrom < usize >) -> & mut str { & mut self [..] [index] } }
};
}
