// Generated macro for wait_for_counter (function)
macro_rules! Depcrate_thread_pool_testwait_for_counter {
() => {
// Module: crate::thread_pool::test
// Provides: {"wait_for_counter"}
// Dependencies: {}
# [doc = " Wait until a counter is no longer shared, then return its value."] fn wait_for_counter (mut counter : Arc < AtomicUsize >) -> usize { use std :: { thread , time } ; for _ in 0 .. 60 { counter = match Arc :: try_unwrap (counter) { Ok (counter) => return counter . into_inner () , Err (counter) => { thread :: sleep (time :: Duration :: from_secs (1)) ; counter } } ; } panic ! ("Counter is still shared!") ; }
};
}
