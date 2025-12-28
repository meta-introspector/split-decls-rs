macro_rules! MapConsumer {
    () => {
        struct MapConsumer < 'f , C , F > { base : C , map_op : & 'f F , }
    };
}

MapConsumer!();