macro_rules! SkipAnyConsumer {
    () => {
        struct SkipAnyConsumer < 'f , C > { base : C , count : & 'f AtomicUsize , }
    };
}

SkipAnyConsumer!();