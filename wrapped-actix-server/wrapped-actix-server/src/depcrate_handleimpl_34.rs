// Generated macro for impl_34 (impl)
macro_rules! Depcrate_handleimpl_34 {
() => {
// Module: crate::handle
// Provides: {"impl_34"}
// Dependencies: {}
impl ServerHandle { pub (crate) fn new (cmd_tx : UnboundedSender < ServerCommand >) -> Self { ServerHandle { cmd_tx } } pub (crate) fn worker_faulted (& self , idx : usize) { let _ = self . cmd_tx . send (ServerCommand :: WorkerFaulted (idx)) ; } # [doc = " Pause accepting incoming connections."] # [doc = ""] # [doc = " May drop socket pending connection. All open connections remain active."] pub fn pause (& self) -> impl Future < Output = () > { let (tx , rx) = oneshot :: channel () ; let _ = self . cmd_tx . send (ServerCommand :: Pause (tx)) ; async { let _ = rx . await ; } } # [doc = " Resume accepting incoming connections."] pub fn resume (& self) -> impl Future < Output = () > { let (tx , rx) = oneshot :: channel () ; let _ = self . cmd_tx . send (ServerCommand :: Resume (tx)) ; async { let _ = rx . await ; } } # [doc = " Stop incoming connection processing, stop all workers and exit."] pub fn stop (& self , graceful : bool) -> impl Future < Output = () > { let (tx , rx) = oneshot :: channel () ; let _ = self . cmd_tx . send (ServerCommand :: Stop { graceful , completion : Some (tx) , force_system_stop : false , }) ; async { let _ = rx . await ; } } }
};
}
