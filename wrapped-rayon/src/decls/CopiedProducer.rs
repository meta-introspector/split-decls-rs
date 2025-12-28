macro_rules! CopiedProducer {
    () => {
        struct CopiedProducer < P > { base : P , }
    };
}

CopiedProducer!()