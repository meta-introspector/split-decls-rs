macro_rules! IterMutProducer {
    () => {
        struct IterMutProducer < 'data , T : Send > { slice : & 'data mut [T] , }
    };
}

IterMutProducer!();