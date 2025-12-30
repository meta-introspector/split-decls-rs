// Generated macro for impl_82 (impl)
macro_rules! Depcrate_arbitrary_arraysimpl_82 {
() => {
// Module: crate::arbitrary::arrays
// Provides: {"impl_82"}
// Dependencies: {}
impl < A : Arbitrary , const N : usize > Arbitrary for [A ; N] { type Parameters = A :: Parameters ; type Strategy = UniformArrayStrategy < A :: Strategy , [A ; N] > ; fn arbitrary_with (args : Self :: Parameters) -> Self :: Strategy { let base = any_with :: < A > (args) ; UniformArrayStrategy :: new (base) } }
};
}
