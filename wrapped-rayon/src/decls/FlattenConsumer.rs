macro_rules! FlattenConsumer {
    () => {
        struct FlattenConsumer < C > { base : C , }
    };
}

FlattenConsumer!()