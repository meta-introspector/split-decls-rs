// Generated macro for impl_38 (impl)
macro_rules! Depcrate_macros_helperimpl_38 {
() => {
// Module: crate::macros::helper
// Provides: {"impl_38"}
// Dependencies: {}
impl < I , O > ToResultCall for fn (I) -> O { type Input = I ; type Output = O ; type Error = Infallible ; fn __to_result_call (& self , input : Self :: Input) -> Result < Self :: Output , Self :: Error > { Ok (self (input)) } }
};
}
