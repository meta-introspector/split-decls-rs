macro_rules! deps {
    () => {
        Read!();
    };
}

macro_rules! leb64_from_read {
    () => {
        deps!();
        # [doc = " Decode variable int numbers from a `Read` implementation."] # [doc = ""] # [doc = " Note: currently overflow checks are only done in debug mode."] # [inline] pub fn leb64_from_read (mut r : impl Read) -> Result < (u64 , usize) , std :: io :: Error > { let mut b = [0u8 ; 1] ; let mut i = 0 ; r . read_exact (& mut b) ? ; i += 1 ; let mut value = u64 :: from (b [0]) & 0x7f ; while b [0] & 0x80 != 0 { r . read_exact (& mut b) ? ; i += 1 ; debug_assert ! (i <= 10 , "Would overflow value at 11th iteration") ; value += 1 ; value = (value << 7) + (u64 :: from (b [0]) & 0x7f) ; } Ok ((value , i)) }
    };
}

leb64_from_read!();