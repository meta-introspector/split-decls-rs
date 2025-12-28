macro_rules! TakeAnyConsumer {
    () => {
        struct TakeAnyConsumer < 'f , C > { base : C , count : & 'f AtomicUsize , }
    };
}

TakeAnyConsumer!()