macro_rules! IterProducer {
    () => {
        struct IterProducer < 'data , T : Sync > { slice : & 'data [T] , }
    };
}

IterProducer!()