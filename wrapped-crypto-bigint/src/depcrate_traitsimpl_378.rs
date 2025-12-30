// Generated macro for impl_378 (impl)
macro_rules! Depcrate_traitsimpl_378 {
() => {
// Module: crate::traits
// Provides: {"impl_378"}
// Dependencies: {}
# [allow (deprecated)] impl < T , Rhs > WideningMul < Rhs > for T where T : ConcatenatingMul < Rhs > , { type Output = < T as ConcatenatingMul < Rhs > > :: Output ; fn widening_mul (& self , rhs : Rhs) -> Self :: Output { self . concatenating_mul (rhs) } }
};
}
