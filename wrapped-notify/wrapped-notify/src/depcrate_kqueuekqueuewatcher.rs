// Generated macro for KqueueWatcher (struct)
macro_rules! Depcrate_kqueueKqueueWatcher {
() => {
// Module: crate::kqueue
// Provides: {"KqueueWatcher"}
// Dependencies: {}
# [doc = " Watcher implementation based on inotify"] # [derive (Debug)] pub struct KqueueWatcher { channel : Sender < EventLoopMsg > , waker : Arc < mio :: Waker > , }
};
}
