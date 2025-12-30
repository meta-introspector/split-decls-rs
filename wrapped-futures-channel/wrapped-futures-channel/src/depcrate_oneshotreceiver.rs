// Generated macro for Receiver (struct)
macro_rules! Depcrate_oneshotReceiver {
() => {
// Module: crate::oneshot
// Provides: {"Receiver"}
// Dependencies: {}
# [doc = " A future for a value that will be provided by another asynchronous task."] # [doc = ""] # [doc = " This is created by the [`channel`] function."] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Receiver < T > { inner : Arc < Inner < T > > , }
};
}
