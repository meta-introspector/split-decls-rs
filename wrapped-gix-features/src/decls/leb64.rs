macro_rules! leb64 {
    () => {
        # [doc = " Decode variable int numbers."] # [inline] pub fn leb64 (d : & [u8]) -> (u64 , usize) { let mut i = 0 ; let mut c = d [i] ; i += 1 ; let mut value = u64 :: from (c) & 0x7f ; while c & 0x80 != 0 { c = d [i] ; i += 1 ; debug_assert ! (i <= 10 , "Would overflow value at 11th iteration") ; value += 1 ; value = (value << 7) + (u64 :: from (c) & 0x7f) ; } (value , i) }
    };
}

leb64!();