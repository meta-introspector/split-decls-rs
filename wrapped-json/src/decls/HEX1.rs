macro_rules! HEX1 {
    () => {
        static HEX1 : [i16 ; 256] = build_hex_table (4) ;
    };
}

HEX1!();