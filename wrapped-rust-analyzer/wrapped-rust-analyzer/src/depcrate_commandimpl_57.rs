// Generated macro for impl_57 (impl)
macro_rules! Depcrate_commandimpl_57 {
() => {
// Module: crate::command
// Provides: {"impl_57"}
// Dependencies: {}
impl < T : Sized + Send + 'static > CargoActor < T > { fn new (parser : impl CargoParser < T > , sender : Sender < T > , stdout : ChildStdout , stderr : ChildStderr ,) -> Self { let parser = Box :: new (parser) ; CargoActor { parser , sender , stdout , stderr } } }
};
}
