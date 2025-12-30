// Generated macro for remote_into_raw (function)
macro_rules! Depcrate_remoteremote_into_raw {
() => {
// Module: crate::remote
// Provides: {"remote_into_raw"}
// Dependencies: {}
pub fn remote_into_raw (remote : Remote < '_ >) -> * mut raw :: git_remote { let ret = remote . raw ; mem :: forget (remote) ; ret }
};
}
