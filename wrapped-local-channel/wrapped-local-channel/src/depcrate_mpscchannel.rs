// Generated macro for channel (function)
macro_rules! Depcrate_mpscchannel {
() => {
// Module: crate::mpsc
// Provides: {"channel"}
// Dependencies: {}
# [doc = " Creates a unbounded in-memory channel with buffered storage."] # [doc = ""] # [doc = " [Sender]s and [Receiver]s are `!Send`."] pub fn channel < T > () -> (Sender < T > , Receiver < T >) { let shared = Rc :: new (RefCell :: new (Shared { has_receiver : true , buffer : VecDeque :: new () , blocked_recv : LocalWaker :: new () , })) ; let sender = Sender { shared : shared . clone () , } ; let receiver = Receiver { shared } ; (sender , receiver) }
};
}
