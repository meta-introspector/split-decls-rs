macro_rules! DrainProducer {
    () => {
        pub (crate) struct DrainProducer < 'data , T : Send > { slice : & 'data mut [T] , }
    };
}

DrainProducer!();