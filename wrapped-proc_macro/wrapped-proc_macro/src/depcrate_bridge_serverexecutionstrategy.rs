// Generated macro for ExecutionStrategy (trait)
macro_rules! Depcrate_bridge_serverExecutionStrategy {
() => {
// Module: crate::bridge::server
// Provides: {"ExecutionStrategy"}
// Dependencies: {}
pub trait ExecutionStrategy { fn run_bridge_and_client (& self , dispatcher : & mut impl DispatcherTrait , input : Buffer , run_client : extern "C" fn (BridgeConfig < '_ >) -> Buffer , force_show_panics : bool ,) -> Buffer ; }
};
}
