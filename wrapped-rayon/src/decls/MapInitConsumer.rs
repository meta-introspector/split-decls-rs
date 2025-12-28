macro_rules! MapInitConsumer {
    () => {
        struct MapInitConsumer < 'f , C , INIT , F > { base : C , init : & 'f INIT , map_op : & 'f F , }
    };
}

MapInitConsumer!()