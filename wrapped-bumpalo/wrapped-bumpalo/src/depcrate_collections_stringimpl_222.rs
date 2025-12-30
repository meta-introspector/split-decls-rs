// Generated macro for impl_222 (impl)
macro_rules! Depcrate_collections_stringimpl_222 {
() => {
// Module: crate::collections::string
// Provides: {"impl_222"}
// Dependencies: {}
impl < 'bump > ops :: Index < ops :: RangeToInclusive < usize > > for String < 'bump > { type Output = str ; # [inline] fn index (& self , index : ops :: RangeToInclusive < usize >) -> & str { Index :: index (& * * self , index) } }
};
}
