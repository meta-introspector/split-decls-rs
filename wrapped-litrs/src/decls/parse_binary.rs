macro_rules! parse_binary {
    () => {
        # [test] fn parse_binary () { check ("0b0" , 0b0 , Binary , "0" , None) ; check ("0b000" , 0b000 , Binary , "000" , None) ; check ("0b1" , 0b1 , Binary , "1" , None) ; check ("0b01" , 0b01 , Binary , "01" , None) ; check ("0b101010" , 0b101010 , Binary , "101010" , None) ; check ("0b10_10_10" , 0b10_10_10 , Binary , "10_10_10" , None) ; check ("0b01101110____" , 0b01101110____ , Binary , "01101110____" , None) ; check ("0b10010u8" , 0b10010u8 , Binary , "10010" , Some (Ty :: U8)) ; check ("0b10010i8" , 0b10010u8 , Binary , "10010" , Some (Ty :: I8)) ; check ("0b10010u64" , 0b10010u64 , Binary , "10010" , Some (Ty :: U64)) ; check ("0b10010i64" , 0b10010i64 , Binary , "10010" , Some (Ty :: I64)) ; check ("0b1011001_00110000_00101000_10100101u32" , 0b1011001_00110000_00101000_10100101u32 , Binary , "1011001_00110000_00101000_10100101" , Some (Ty :: U32) ,) ; }
    };
}

parse_binary!()