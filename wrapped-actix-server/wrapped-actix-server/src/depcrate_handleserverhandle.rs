// Generated macro for ServerHandle (struct)
macro_rules! Depcrate_handleServerHandle {
() => {
// Module: crate::handle
// Provides: {"ServerHandle"}
// Dependencies: {}
# [doc = " Server handle."] # [derive (Debug , Clone)] pub struct ServerHandle { cmd_tx : UnboundedSender < ServerCommand > , }
};
}
