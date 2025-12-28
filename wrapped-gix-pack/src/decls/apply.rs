macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! apply {
    () => {
        deps!();
        pub (crate) fn apply (base : & [u8] , mut target : & mut [u8] , data : & [u8]) -> Result < () , apply :: Error > { let mut i = 0 ; while let Some (cmd) = data . get (i) { i += 1 ; match cmd { cmd if cmd & 0b1000_0000 != 0 => { let (mut ofs , mut size) : (u32 , u32) = (0 , 0) ; if cmd & 0b0000_0001 != 0 { ofs = u32 :: from (data [i]) ; i += 1 ; } if cmd & 0b0000_0010 != 0 { ofs |= u32 :: from (data [i]) << 8 ; i += 1 ; } if cmd & 0b0000_0100 != 0 { ofs |= u32 :: from (data [i]) << 16 ; i += 1 ; } if cmd & 0b0000_1000 != 0 { ofs |= u32 :: from (data [i]) << 24 ; i += 1 ; } if cmd & 0b0001_0000 != 0 { size = u32 :: from (data [i]) ; i += 1 ; } if cmd & 0b0010_0000 != 0 { size |= u32 :: from (data [i]) << 8 ; i += 1 ; } if cmd & 0b0100_0000 != 0 { size |= u32 :: from (data [i]) << 16 ; i += 1 ; } if size == 0 { size = 0x10000 ; } let ofs = ofs as usize ; std :: io :: Write :: write (& mut target , & base [ofs .. ofs + size as usize]) . map_err (| _e | apply :: Error :: DeltaCopyBaseSliceMismatch) ? ; } 0 => return Err (apply :: Error :: UnsupportedCommandCode) , size => { std :: io :: Write :: write (& mut target , & data [i .. i + * size as usize]) . map_err (| _e | apply :: Error :: DeltaCopyDataSliceMismatch) ? ; i += * size as usize ; } } } assert_eq ! (i , data . len ()) ; assert_eq ! (target . len () , 0) ; Ok (()) }
    };
}

apply!();