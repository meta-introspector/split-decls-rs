// Generated macro for Inotify (struct)
macro_rules! Depcrate_inotifyInotify {
() => {
// Module: crate::inotify
// Provides: {"Inotify"}
// Dependencies: {}
# [doc = " Idiomatic Rust wrapper around Linux's inotify API"] # [doc = ""] # [doc = " `Inotify` is a wrapper around an inotify instance. It generally tries to"] # [doc = " adhere to the underlying inotify API closely, while making access to it"] # [doc = " safe and convenient."] # [doc = ""] # [doc = " Please refer to the [top-level documentation] for further details and a"] # [doc = " usage example."] # [doc = ""] # [doc = " [top-level documentation]: crate"] # [derive (Debug)] pub struct Inotify { fd : Arc < FdGuard > , }
};
}
