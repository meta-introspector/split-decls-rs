macro_rules! noop_raw_waker {
    () => {
        const fn noop_raw_waker () -> RawWaker { RawWaker :: new (null () , & NOOP_WAKER_VTABLE) }
    };
}

noop_raw_waker!();