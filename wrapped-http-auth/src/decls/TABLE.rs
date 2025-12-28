macro_rules! TABLE {
    () => {
        static TABLE : [u8 ; 128] = build_table () ;
    };
}

TABLE!();