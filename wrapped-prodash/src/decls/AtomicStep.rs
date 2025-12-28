macro_rules! AtomicStep {
    () => {
        # [doc = " The amount of steps a progress can make, for threadsafe counting."] pub type AtomicStep = AtomicUsize ;
    };
}

AtomicStep!();