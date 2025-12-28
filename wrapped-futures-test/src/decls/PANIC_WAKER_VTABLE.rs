macro_rules! PANIC_WAKER_VTABLE {
    () => {
        const PANIC_WAKER_VTABLE : RawWakerVTable = RawWakerVTable :: new (clone_panic_waker , wake_panic , wake_panic , noop) ;
    };
}

PANIC_WAKER_VTABLE!()