// Generated macro for impl_227 (impl)
macro_rules! Depcrate_collections_stringimpl_227 {
() => {
// Module: crate::collections::string
// Provides: {"impl_227"}
// Dependencies: {}
impl < 'bump > ops :: IndexMut < ops :: RangeInclusive < usize > > for String < 'bump > { # [inline] fn index_mut (& mut self , index : ops :: RangeInclusive < usize >) -> & mut str { IndexMut :: index_mut (& mut * * self , index) } }
};
}
