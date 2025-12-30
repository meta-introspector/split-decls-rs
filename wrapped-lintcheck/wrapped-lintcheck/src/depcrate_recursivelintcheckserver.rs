// Generated macro for LintcheckServer (struct)
macro_rules! Depcrate_recursiveLintcheckServer {
() => {
// Module: crate::recursive
// Provides: {"LintcheckServer"}
// Dependencies: {}
pub (crate) struct LintcheckServer { pub local_addr : SocketAddr , receiver : Receiver < ClippyWarning > , sender : Arc < Sender < ClippyWarning > > , }
};
}
