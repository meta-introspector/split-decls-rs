// Generated macro for impl_42 (impl)
macro_rules! Depcrate_macros_helperimpl_42 {
() => {
// Module: crate::macros::helper
// Provides: {"impl_42"}
// Dependencies: {}
impl < I , O , S > ToScalarValueCall < S > for & fn (I) -> O where S : ScalarValue , O : ToScalarValue < S > , { type Input = I ; fn __to_scalar_value_call (& self , input : Self :: Input) -> S { self (input) . to_scalar_value () } }
};
}
