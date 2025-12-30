// Generated macro for impl_43 (impl)
macro_rules! Depcrate_macros_helperimpl_43 {
() => {
// Module: crate::macros::helper
// Provides: {"impl_43"}
// Dependencies: {}
impl < I , O , S > ToScalarValueCall < S > for fn (I) -> O where S : ScalarValue , O : Display , { type Input = I ; fn __to_scalar_value_call (& self , input : Self :: Input) -> S { S :: from_displayable_non_static (& self (input)) } }
};
}
