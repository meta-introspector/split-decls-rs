// Generated macro for impl_221 (impl)
macro_rules! Depcrate_collections_stringimpl_221 {
() => {
// Module: crate::collections::string
// Provides: {"impl_221"}
// Dependencies: {}
impl < 'bump > ops :: Index < ops :: RangeInclusive < usize > > for String < 'bump > { type Output = str ; # [inline] fn index (& self , index : ops :: RangeInclusive < usize >) -> & str { Index :: index (& * * self , index) } }
};
}
