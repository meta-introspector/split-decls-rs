// Generated macro for impl_62 (impl)
macro_rules! Depcrate_commandimpl_62 {
() => {
// Module: crate::command
// Provides: {"impl_62"}
// Dependencies: {}
impl < T : Sized + Send + 'static > CargoActor < T > { fn new (parser : impl CargoParser < T > , sender : Sender < T > , stdout : ChildStdout , stderr : ChildStderr ,) -> Self { let parser = Box :: new (parser) ; CargoActor { parser , sender , stdout , stderr } } }
};
}
