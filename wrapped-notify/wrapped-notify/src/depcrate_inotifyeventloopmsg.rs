// Generated macro for EventLoopMsg (enum)
macro_rules! Depcrate_inotifyEventLoopMsg {
() => {
// Module: crate::inotify
// Provides: {"EventLoopMsg"}
// Dependencies: {}
enum EventLoopMsg { AddWatch (PathBuf , RecursiveMode , Sender < Result < () > >) , RemoveWatch (PathBuf , Sender < Result < () > >) , Shutdown , Configure (Config , BoundSender < Result < bool > >) , }
};
}
