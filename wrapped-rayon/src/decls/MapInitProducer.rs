macro_rules! MapInitProducer {
    () => {
        struct MapInitProducer < 'f , P , INIT , F > { base : P , init : & 'f INIT , map_op : & 'f F , }
    };
}

MapInitProducer!();