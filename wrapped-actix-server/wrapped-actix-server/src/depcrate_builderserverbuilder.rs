// Generated macro for ServerBuilder (struct)
macro_rules! Depcrate_builderServerBuilder {
() => {
// Module: crate::builder
// Provides: {"ServerBuilder"}
// Dependencies: {}
# [doc = " [Server] builder."] pub struct ServerBuilder { pub (crate) threads : usize , pub (crate) token : usize , pub (crate) backlog : u32 , pub (crate) factories : Vec < Box < dyn InternalServiceFactory > > , pub (crate) sockets : Vec < (usize , String , MioListener) > , pub (crate) mptcp : MpTcp , pub (crate) exit : bool , pub (crate) listen_os_signals : bool , pub (crate) shutdown_signal : Option < BoxFuture < 'static , () > > , pub (crate) cmd_tx : UnboundedSender < ServerCommand > , pub (crate) cmd_rx : UnboundedReceiver < ServerCommand > , pub (crate) worker_config : ServerWorkerConfig , }
};
}
