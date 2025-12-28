macro_rules! MapWithProducer {
    () => {
        struct MapWithProducer < 'f , P , U , F > { base : P , item : U , map_op : & 'f F , }
    };
}

MapWithProducer!();