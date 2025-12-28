macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! sc_reject_noncanonical {
    () => {
        deps!();
        pub fn sc_reject_noncanonical (s : & [u8]) -> Result < () , Error > { static L : [u8 ; 32] = [0xed , 0xd3 , 0xf5 , 0x5c , 0x1a , 0x63 , 0x12 , 0x58 , 0xd6 , 0x9c , 0xf7 , 0xa2 , 0xde , 0xf9 , 0xde , 0x14 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x10 ,] ; if s . len () != 32 { panic ! ("Invalid compressed length") } let mut c : u8 = 0 ; let mut n : u8 = 1 ; let mut i = 31 ; loop { c |= ((((s [i] as i32) - (L [i] as i32)) >> 8) as u8) & n ; n &= ((((s [i] ^ L [i]) as i32) - 1) >> 8) as u8 ; if i == 0 { break ; } i -= 1 ; } if c != 0 { Ok (()) } else { Err (Error :: NonCanonical) } }
    };
}

sc_reject_noncanonical!()