macro_rules! FlatMapConsumer {
    () => {
        struct FlatMapConsumer < 'f , C , F > { base : C , map_op : & 'f F , }
    };
}

FlatMapConsumer!();