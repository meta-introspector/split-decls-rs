macro_rules! RevProducer {
    () => {
        struct RevProducer < P > { base : P , len : usize , }
    };
}

RevProducer!();