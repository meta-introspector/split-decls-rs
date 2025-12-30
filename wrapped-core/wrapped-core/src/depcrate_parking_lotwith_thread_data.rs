// Generated macro for with_thread_data (function)
macro_rules! Depcrate_parking_lotwith_thread_data {
() => {
// Module: crate::parking_lot
// Provides: {"with_thread_data"}
// Dependencies: {}
# [inline (always)] fn with_thread_data < T > (f : impl FnOnce (& ThreadData) -> T) -> T { let mut thread_data_storage = None ; thread_local ! (static THREAD_DATA : ThreadData = ThreadData :: new ()) ; let thread_data_ptr = THREAD_DATA . try_with (| x | x as * const ThreadData) . unwrap_or_else (| _ | thread_data_storage . get_or_insert_with (ThreadData :: new)) ; f (unsafe { & * thread_data_ptr }) }
};
}
