macro_rules! FilterFolder {
    () => {
        struct FilterFolder < 'p , C , P > { base : C , filter_op : & 'p P , }
    };
}

FilterFolder!();