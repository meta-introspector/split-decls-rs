macro_rules! DefaultRandomSource {
    () => {
        struct DefaultRandomSource { counter : AtomicUsize , }
    };
}

DefaultRandomSource!()