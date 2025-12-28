macro_rules! clone_panic_waker {
    () => {
        unsafe fn clone_panic_waker (_data : * const ()) -> RawWaker { raw_panic_waker () }
    };
}

clone_panic_waker!();