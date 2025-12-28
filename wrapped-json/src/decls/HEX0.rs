macro_rules! HEX0 {
    () => {
        static HEX0 : [i16 ; 256] = build_hex_table (0) ;
    };
}

HEX0!();