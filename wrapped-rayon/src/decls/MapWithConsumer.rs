macro_rules! MapWithConsumer {
    () => {
        struct MapWithConsumer < 'f , C , U , F > { base : C , item : U , map_op : & 'f F , }
    };
}

MapWithConsumer!()