macro_rules! WhileSomeConsumer {
    () => {
        struct WhileSomeConsumer < 'f , C > { base : C , full : & 'f AtomicBool , }
    };
}

WhileSomeConsumer!();