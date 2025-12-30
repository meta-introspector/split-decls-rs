// Generated macro for impl_36 (impl)
macro_rules! Depcrate_deriveimpl_36 {
() => {
// Module: crate::derive
// Provides: {"impl_36"}
// Dependencies: {}
impl < T : CommandFactory > CommandFactory for Box < T > { fn command () -> Command { < T as CommandFactory > :: command () } fn command_for_update () -> Command { < T as CommandFactory > :: command_for_update () } }
};
}
