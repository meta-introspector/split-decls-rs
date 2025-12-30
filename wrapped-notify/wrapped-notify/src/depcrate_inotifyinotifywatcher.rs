// Generated macro for INotifyWatcher (struct)
macro_rules! Depcrate_inotifyINotifyWatcher {
() => {
// Module: crate::inotify
// Provides: {"INotifyWatcher"}
// Dependencies: {}
# [doc = " Watcher implementation based on inotify"] # [derive (Debug)] pub struct INotifyWatcher { channel : Sender < EventLoopMsg > , waker : Arc < mio :: Waker > , }
};
}
