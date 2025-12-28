macro_rules! StepByProducer {
    () => {
        struct StepByProducer < P > { base : P , step : usize , len : usize , }
    };
}

StepByProducer!()