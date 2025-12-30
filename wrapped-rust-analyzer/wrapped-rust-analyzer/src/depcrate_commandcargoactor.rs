// Generated macro for CargoActor (struct)
macro_rules! Depcrate_commandCargoActor {
() => {
// Module: crate::command
// Provides: {"CargoActor"}
// Dependencies: {}
struct CargoActor < T > { parser : Box < dyn CargoParser < T > > , sender : Sender < T > , stdout : ChildStdout , stderr : ChildStderr , }
};
}
