macro_rules! noop_clone {
    () => {
        unsafe fn noop_clone (_data : * const ()) -> RawWaker { noop_raw_waker () }
    };
}

noop_clone!()