macro_rules! Shared {
    () => {
        struct Shared { value : AtomicUsize , waker : AtomicWaker , }
    };
}

Shared!()