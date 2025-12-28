macro_rules! MapWithFolder {
    () => {
        struct MapWithFolder < 'f , C , U , F > { base : C , item : U , map_op : & 'f F , }
    };
}

MapWithFolder!()