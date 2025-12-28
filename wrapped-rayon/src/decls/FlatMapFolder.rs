macro_rules! FlatMapFolder {
    () => {
        struct FlatMapFolder < 'f , C , F , R > { base : C , map_op : & 'f F , previous : Option < R > , }
    };
}

FlatMapFolder!()