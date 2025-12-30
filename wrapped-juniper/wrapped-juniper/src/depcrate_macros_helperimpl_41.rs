// Generated macro for impl_41 (impl)
macro_rules! Depcrate_macros_helperimpl_41 {
() => {
// Module: crate::macros::helper
// Provides: {"impl_41"}
// Dependencies: {}
impl < I , S > ToScalarValueCall < S > for & & fn (I) -> String where S : ScalarValue , { type Input = I ; fn __to_scalar_value_call (& self , input : Self :: Input) -> S { self (input) . into () } }
};
}
