// Generated macro for Receiver (struct)
macro_rules! Depcrate_mpscReceiver {
() => {
// Module: crate::mpsc
// Provides: {"Receiver"}
// Dependencies: {}
# [doc = " The receiving end of a bounded mpsc channel."] # [doc = ""] # [doc = " This value is created by the [`channel`] function."] pub struct Receiver < T > { inner : Option < Arc < BoundedInner < T > > > , }
};
}
