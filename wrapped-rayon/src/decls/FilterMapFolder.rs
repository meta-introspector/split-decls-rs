macro_rules! FilterMapFolder {
    () => {
        struct FilterMapFolder < 'p , C , P > { base : C , filter_op : & 'p P , }
    };
}

FilterMapFolder!()