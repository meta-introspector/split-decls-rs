macro_rules! SkipAnyWhileConsumer {
    () => {
        struct SkipAnyWhileConsumer < 'p , C , P > { base : C , predicate : & 'p P , skipping : & 'p AtomicBool , }
    };
}

SkipAnyWhileConsumer!();