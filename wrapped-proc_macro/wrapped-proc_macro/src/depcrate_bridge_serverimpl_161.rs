// Generated macro for impl_161 (impl)
macro_rules! Depcrate_bridge_serverimpl_161 {
() => {
// Module: crate::bridge::server
// Provides: {"impl_161"}
// Dependencies: {}
impl < P > ExecutionStrategy for MaybeCrossThread < P > where P : MessagePipe < Buffer > + Send + 'static , { fn run_bridge_and_client (& self , dispatcher : & mut impl DispatcherTrait , input : Buffer , run_client : extern "C" fn (BridgeConfig < '_ >) -> Buffer , force_show_panics : bool ,) -> Buffer { if self . cross_thread || ALREADY_RUNNING_SAME_THREAD . get () { < CrossThread < P > > :: new () . run_bridge_and_client (dispatcher , input , run_client , force_show_panics ,) } else { SameThread . run_bridge_and_client (dispatcher , input , run_client , force_show_panics) } } }
};
}
