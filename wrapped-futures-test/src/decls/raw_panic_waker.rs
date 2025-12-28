macro_rules! raw_panic_waker {
    () => {
        const fn raw_panic_waker () -> RawWaker { RawWaker :: new (null () , & PANIC_WAKER_VTABLE) }
    };
}

raw_panic_waker!()