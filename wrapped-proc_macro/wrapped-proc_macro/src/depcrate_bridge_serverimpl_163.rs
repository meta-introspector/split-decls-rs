// Generated macro for impl_163 (impl)
macro_rules! Depcrate_bridge_serverimpl_163 {
() => {
// Module: crate::bridge::server
// Provides: {"impl_163"}
// Dependencies: {}
impl ExecutionStrategy for SameThread { fn run_bridge_and_client (& self , dispatcher : & mut impl DispatcherTrait , input : Buffer , run_client : extern "C" fn (BridgeConfig < '_ >) -> Buffer , force_show_panics : bool ,) -> Buffer { let _guard = RunningSameThreadGuard :: new () ; let mut dispatch = | buf | dispatcher . dispatch (buf) ; run_client (BridgeConfig { input , dispatch : (& mut dispatch) . into () , force_show_panics , _marker : marker :: PhantomData , }) } }
};
}
