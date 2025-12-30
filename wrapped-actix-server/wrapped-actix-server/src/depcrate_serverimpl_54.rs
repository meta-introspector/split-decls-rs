// Generated macro for impl_54 (impl)
macro_rules! Depcrate_serverimpl_54 {
() => {
// Module: crate::server
// Provides: {"impl_54"}
// Dependencies: {}
impl Server { # [doc = " Create server build."] pub fn build () -> ServerBuilder { ServerBuilder :: default () } pub (crate) fn new (builder : ServerBuilder) -> Self { Server { handle : ServerHandle :: new (builder . cmd_tx . clone ()) , fut : Box :: pin (ServerInner :: run (builder)) , } } # [doc = " Get a `Server` handle that can be used issue commands and change it's state."] # [doc = ""] # [doc = " See [ServerHandle](ServerHandle) for usage."] pub fn handle (& self) -> ServerHandle { self . handle . clone () } }
};
}
