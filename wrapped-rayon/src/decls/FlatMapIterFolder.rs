macro_rules! FlatMapIterFolder {
    () => {
        struct FlatMapIterFolder < 'f , C , F > { base : C , map_op : & 'f F , }
    };
}

FlatMapIterFolder!()