// Generated macro for channel (function)
macro_rules! Depcrate_common_watchchannel {
() => {
// Module: crate::common::watch
// Provides: {"channel"}
// Dependencies: {}
pub (crate) fn channel (initial : Value) -> (Sender , Receiver) { debug_assert ! (initial != CLOSED , "watch::channel initial state of 0 is reserved") ; let shared = Arc :: new (Shared { value : AtomicUsize :: new (initial) , waker : AtomicWaker :: new () , }) ; (Sender { shared : shared . clone () , } , Receiver { shared } ,) }
};
}
