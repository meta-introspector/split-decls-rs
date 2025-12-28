macro_rules! deps {
    () => {
        RawThreadId!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        unsafe impl GetThreadId for RawThreadId { const INIT : RawThreadId = RawThreadId ; fn nonzero_thread_id (& self) -> NonZeroUsize { thread_local ! (static KEY : u8 = 0) ; KEY . with (| x | { NonZeroUsize :: new (x as * const _ as usize) . expect ("thread-local variable address is null") }) } }
    };
}

impl_76!()