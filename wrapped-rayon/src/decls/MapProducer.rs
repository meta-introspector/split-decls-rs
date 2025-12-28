macro_rules! MapProducer {
    () => {
        struct MapProducer < 'f , P , F > { base : P , map_op : & 'f F , }
    };
}

MapProducer!()