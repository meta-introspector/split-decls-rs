// Generated macro for ServerInner (struct)
macro_rules! Depcrate_serverServerInner {
() => {
// Module: crate::server
// Provides: {"ServerInner"}
// Dependencies: {}
pub struct ServerInner { worker_handles : Vec < WorkerHandleServer > , accept_handle : Option < thread :: JoinHandle < () > > , worker_config : ServerWorkerConfig , services : Vec < Box < dyn InternalServiceFactory > > , waker_queue : WakerQueue , system_stop : bool , stopping : bool , }
};
}
