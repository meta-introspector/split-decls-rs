macro_rules! noop_waker {
    () => {
        # [cfg (feature = "client")] fn noop_waker () -> Waker { const NOOP_RAW_WAKER : RawWaker = RawWaker :: new (std :: ptr :: null () , & NOOP_VTABLE) ; const NOOP_VTABLE : RawWakerVTable = RawWakerVTable :: new (| _ : * const () | NOOP_RAW_WAKER , | _ : * const () | { } , | _ : * const () | { } , | _ : * const () | { } ,) ; unsafe { Waker :: from_raw (NOOP_RAW_WAKER) } }
    };
}

noop_waker!();