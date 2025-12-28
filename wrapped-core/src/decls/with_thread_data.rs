macro_rules! deps {
    () => {
        ThreadData!();
    };
}

macro_rules! with_thread_data {
    () => {
        deps!();
        # [inline] pub fn with_thread_data < T > (f : impl FnOnce (& ThreadData) -> T) -> T { let mut thread_data_ptr = ptr :: null () ; if ! ThreadParker :: IS_CHEAP_TO_CONSTRUCT { thread_local ! (static THREAD_DATA : ThreadData = ThreadData :: new ()) ; if let Ok (tls_thread_data) = THREAD_DATA . try_with (| x | x as * const ThreadData) { thread_data_ptr = tls_thread_data ; } } let mut thread_data_storage = None ; if thread_data_ptr . is_null () { thread_data_ptr = thread_data_storage . get_or_insert_with (ThreadData :: new) ; } f (unsafe { & * thread_data_ptr }) }
    };
}

with_thread_data!();