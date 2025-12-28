macro_rules! MapWithIter {
    () => {
        struct MapWithIter < 'f , I , U , F > { base : I , item : U , map_op : & 'f F , }
    };
}

MapWithIter!();