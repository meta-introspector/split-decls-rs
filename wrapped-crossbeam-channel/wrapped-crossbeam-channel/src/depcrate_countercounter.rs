// Generated macro for Counter (struct)
macro_rules! Depcrate_counterCounter {
() => {
// Module: crate::counter
// Provides: {"Counter"}
// Dependencies: {}
# [doc = " Reference counter internals."] struct Counter < C > { # [doc = " The number of senders associated with the channel."] senders : AtomicUsize , # [doc = " The number of receivers associated with the channel."] receivers : AtomicUsize , # [doc = " Set to `true` if the last sender or the last receiver reference deallocates the channel."] destroy : AtomicBool , # [doc = " The internal channel."] chan : C , }
};
}
