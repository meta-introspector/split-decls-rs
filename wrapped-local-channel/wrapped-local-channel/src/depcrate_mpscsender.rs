// Generated macro for Sender (struct)
macro_rules! Depcrate_mpscSender {
() => {
// Module: crate::mpsc
// Provides: {"Sender"}
// Dependencies: {}
# [doc = " The transmission end of a channel."] # [doc = ""] # [doc = " This is created by the `channel` function."] # [derive (Debug)] pub struct Sender < T > { shared : Rc < RefCell < Shared < T > > > , }
};
}
