macro_rules! ClonedProducer {
    () => {
        struct ClonedProducer < P > { base : P , }
    };
}

ClonedProducer!();