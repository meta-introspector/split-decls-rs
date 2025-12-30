// Generated macro for impl_82 (impl)
macro_rules! Depcrate_macros_reflectimpl_82 {
() => {
// Module: crate::macros::reflect
// Provides: {"impl_82"}
// Dependencies: {}
impl < S , T : WrappedType < S > , const N : usize > WrappedType < S > for [T ; N] { const VALUE : u128 = T :: VALUE * 10 + 3 ; }
};
}
