// Generated macro for impl_224 (impl)
macro_rules! Depcrate_collections_stringimpl_224 {
() => {
// Module: crate::collections::string
// Provides: {"impl_224"}
// Dependencies: {}
impl < 'bump > ops :: IndexMut < ops :: RangeTo < usize > > for String < 'bump > { # [inline] fn index_mut (& mut self , index : ops :: RangeTo < usize >) -> & mut str { & mut self [..] [index] } }
};
}
