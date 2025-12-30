// Generated macro for impl_37 (impl)
macro_rules! Depcrate_macros_helperimpl_37 {
() => {
// Module: crate::macros::helper
// Provides: {"impl_37"}
// Dependencies: {}
impl < I , O , E > ToResultCall for & fn (I) -> Result < O , E > { type Input = I ; type Output = O ; type Error = E ; fn __to_result_call (& self , input : Self :: Input) -> Result < Self :: Output , Self :: Error > { self (input) } }
};
}
