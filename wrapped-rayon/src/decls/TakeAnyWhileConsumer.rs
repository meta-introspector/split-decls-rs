macro_rules! TakeAnyWhileConsumer {
    () => {
        struct TakeAnyWhileConsumer < 'p , C , P > { base : C , predicate : & 'p P , taking : & 'p AtomicBool , }
    };
}

TakeAnyWhileConsumer!();