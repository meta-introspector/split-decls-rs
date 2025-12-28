macro_rules! MapFolder {
    () => {
        struct MapFolder < 'f , C , F > { base : C , map_op : & 'f F , }
    };
}

MapFolder!()