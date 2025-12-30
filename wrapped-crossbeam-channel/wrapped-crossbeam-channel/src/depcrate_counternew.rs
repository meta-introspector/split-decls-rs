// Generated macro for new (function)
macro_rules! Depcrate_counternew {
() => {
// Module: crate::counter
// Provides: {"new"}
// Dependencies: {}
# [doc = " Wraps a channel into the reference counter."] pub (crate) fn new < C > (chan : C) -> (Sender < C > , Receiver < C >) { let counter = NonNull :: from (Box :: leak (Box :: new (Counter { senders : AtomicUsize :: new (1) , receivers : AtomicUsize :: new (1) , destroy : AtomicBool :: new (false) , chan , }))) ; let s = Sender { counter } ; let r = Receiver { counter } ; (s , r) }
};
}
