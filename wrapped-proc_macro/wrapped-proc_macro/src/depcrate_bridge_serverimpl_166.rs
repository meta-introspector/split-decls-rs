// Generated macro for impl_166 (impl)
macro_rules! Depcrate_bridge_serverimpl_166 {
() => {
// Module: crate::bridge::server
// Provides: {"impl_166"}
// Dependencies: {}
impl < P > ExecutionStrategy for CrossThread < P > where P : MessagePipe < Buffer > + Send + 'static , { fn run_bridge_and_client (& self , dispatcher : & mut impl DispatcherTrait , input : Buffer , run_client : extern "C" fn (BridgeConfig < '_ >) -> Buffer , force_show_panics : bool ,) -> Buffer { let (mut server , mut client) = P :: new () ; let join_handle = thread :: spawn (move | | { let mut dispatch = | b : Buffer | -> Buffer { client . send (b) ; client . recv () . expect ("server died while client waiting for reply") } ; run_client (BridgeConfig { input , dispatch : (& mut dispatch) . into () , force_show_panics , _marker : marker :: PhantomData , }) }) ; while let Some (b) = server . recv () { server . send (dispatcher . dispatch (b)) ; } join_handle . join () . unwrap () } }
};
}
