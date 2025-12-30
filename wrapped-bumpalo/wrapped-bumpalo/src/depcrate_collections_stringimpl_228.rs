// Generated macro for impl_228 (impl)
macro_rules! Depcrate_collections_stringimpl_228 {
() => {
// Module: crate::collections::string
// Provides: {"impl_228"}
// Dependencies: {}
impl < 'bump > ops :: IndexMut < ops :: RangeToInclusive < usize > > for String < 'bump > { # [inline] fn index_mut (& mut self , index : ops :: RangeToInclusive < usize >) -> & mut str { IndexMut :: index_mut (& mut * * self , index) } }
};
}
