// Generated macro for impl_223 (impl)
macro_rules! Depcrate_collections_stringimpl_223 {
() => {
// Module: crate::collections::string
// Provides: {"impl_223"}
// Dependencies: {}
impl < 'bump > ops :: IndexMut < ops :: Range < usize > > for String < 'bump > { # [inline] fn index_mut (& mut self , index : ops :: Range < usize >) -> & mut str { & mut self [..] [index] } }
};
}
