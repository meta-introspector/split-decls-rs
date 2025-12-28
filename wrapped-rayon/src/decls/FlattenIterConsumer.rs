macro_rules! FlattenIterConsumer {
    () => {
        struct FlattenIterConsumer < C > { base : C , }
    };
}

FlattenIterConsumer!();