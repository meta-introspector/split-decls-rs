// Generated macro for Sender (struct)
macro_rules! Depcrate_mpscSender {
() => {
// Module: crate::mpsc
// Provides: {"Sender"}
// Dependencies: {}
# [doc = " The transmission end of a bounded mpsc channel."] # [doc = ""] # [doc = " This value is created by the [`channel`] function."] pub struct Sender < T > (Option < BoundedSenderInner < T > >) ;
};
}
