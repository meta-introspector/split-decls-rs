macro_rules! AbortInner {
    () => {
        # [derive (Debug)] pub (crate) struct AbortInner { pub (crate) waker : AtomicWaker , pub (crate) aborted : AtomicBool , }
    };
}

AbortInner!();