macro_rules! EnumerateProducer {
    () => {
        struct EnumerateProducer < P > { base : P , offset : usize , }
    };
}

EnumerateProducer!()