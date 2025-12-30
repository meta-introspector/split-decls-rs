// Generated macro for MioListener (enum)
macro_rules! Depcrate_socketMioListener {
() => {
// Module: crate::socket
// Provides: {"MioListener"}
// Dependencies: {}
pub (crate) enum MioListener { Tcp (MioTcpListener) , # [cfg (unix)] Uds (MioUnixListener) , }
};
}
