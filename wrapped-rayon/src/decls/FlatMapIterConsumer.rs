macro_rules! FlatMapIterConsumer {
    () => {
        struct FlatMapIterConsumer < 'f , C , F > { base : C , map_op : & 'f F , }
    };
}

FlatMapIterConsumer!()