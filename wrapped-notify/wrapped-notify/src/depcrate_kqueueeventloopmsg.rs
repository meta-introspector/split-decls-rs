// Generated macro for EventLoopMsg (enum)
macro_rules! Depcrate_kqueueEventLoopMsg {
() => {
// Module: crate::kqueue
// Provides: {"EventLoopMsg"}
// Dependencies: {}
enum EventLoopMsg { AddWatch (PathBuf , RecursiveMode , Sender < Result < () > >) , RemoveWatch (PathBuf , Sender < Result < () > >) , Shutdown , }
};
}
