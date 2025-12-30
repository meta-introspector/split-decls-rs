// Generated macro for Sender (struct)
macro_rules! Depcrate_oneshotSender {
() => {
// Module: crate::oneshot
// Provides: {"Sender"}
// Dependencies: {}
# [doc = " A means of transmitting a single value to another task."] # [doc = ""] # [doc = " This is created by the [`channel`] function."] pub struct Sender < T > { inner : Arc < Inner < T > > , }
};
}
