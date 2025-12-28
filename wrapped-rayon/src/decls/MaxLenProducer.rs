macro_rules! MaxLenProducer {
    () => {
        struct MaxLenProducer < P > { base : P , max : usize , }
    };
}

MaxLenProducer!();