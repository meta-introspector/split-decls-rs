// Generated macro for ProxyData (struct)
macro_rules! Depcrate_jobserverProxyData {
() => {
// Module: crate::jobserver
// Provides: {"ProxyData"}
// Dependencies: {}
struct ProxyData { # [doc = " The number of tokens assigned to threads."] # [doc = " If this is 0, a single token is still assigned to this process, but is unused."] used : u16 , # [doc = " The number of threads requesting a token"] pending : u16 , }
};
}
