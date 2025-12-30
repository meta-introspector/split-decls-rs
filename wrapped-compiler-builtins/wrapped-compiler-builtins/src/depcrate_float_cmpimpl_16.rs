// Generated macro for impl_16 (impl)
macro_rules! Depcrate_float_cmpimpl_16 {
() => {
// Module: crate::float::cmp
// Provides: {"impl_16"}
// Dependencies: {}
impl Result { fn to_le_abi (self) -> CmpResult { match self { Result :: Less => - 1 , Result :: Equal => 0 , Result :: Greater => 1 , Result :: Unordered => 1 , } } fn to_ge_abi (self) -> CmpResult { match self { Result :: Less => - 1 , Result :: Equal => 0 , Result :: Greater => 1 , Result :: Unordered => - 1 , } } }
};
}
