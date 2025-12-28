macro_rules! MinLenProducer {
    () => {
        struct MinLenProducer < P > { base : P , min : usize , }
    };
}

MinLenProducer!()