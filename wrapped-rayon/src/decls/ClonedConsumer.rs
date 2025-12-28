macro_rules! ClonedConsumer {
    () => {
        struct ClonedConsumer < C > { base : C , }
    };
}

ClonedConsumer!()