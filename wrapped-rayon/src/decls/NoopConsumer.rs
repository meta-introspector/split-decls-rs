macro_rules! NoopConsumer {
    () => {
        pub (super) struct NoopConsumer ;
    };
}

NoopConsumer!();