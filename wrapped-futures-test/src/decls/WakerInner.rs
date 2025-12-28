macro_rules! WakerInner {
    () => {
        # [derive (Debug)] struct WakerInner { count : AtomicUsize , }
    };
}

WakerInner!();