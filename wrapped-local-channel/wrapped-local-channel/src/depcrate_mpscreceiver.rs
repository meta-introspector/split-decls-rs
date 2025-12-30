// Generated macro for Receiver (struct)
macro_rules! Depcrate_mpscReceiver {
() => {
// Module: crate::mpsc
// Provides: {"Receiver"}
// Dependencies: {}
# [doc = " The receiving end of a channel which implements the `Stream` trait."] # [doc = ""] # [doc = " This is created by the [`channel`] function."] # [derive (Debug)] pub struct Receiver < T > { shared : Rc < RefCell < Shared < T > > > , }
};
}
