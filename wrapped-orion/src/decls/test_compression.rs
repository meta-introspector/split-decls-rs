macro_rules! deps {
    () => {
        FieldElement!();
    };
}

macro_rules! test_compression {
    () => {
        deps!();
        # [cfg (test)] mod test_compression { use super :: * ; use num_rational :: * ; const COMPRESSION_D : [u8 ; 6] = [1 , 4 , 5 , 6 , 10 , 11] ; fn ratcompress (x : i32 , d : u32) -> i32 { let m : i32 = 2i32 . pow (d) ; let mut r = Rational32 :: new (x * m , KYBER_Q as i32) ; r = r . round () % m ; r . to_integer () } fn ratdecompress (y : i32 , d : u32) -> i32 { let m : i32 = 2i32 . pow (d) ; let mut r = Rational32 :: new (y * KYBER_Q as i32 , m) ; r = r . round () ; r . to_integer () } # [test] fn test_compress_with_rational () { for d in COMPRESSION_D { for x in 0 .. KYBER_Q { let fe = FieldElement :: new (x) ; assert_eq ! (fe . compress (d) as i32 , ratcompress (fe . 0 as i32 , d . into ())) ; } } } # [test] fn test_decompress_with_rational () { for d in COMPRESSION_D { for y in 0 .. 2u32 . pow (d as u32) { assert_eq ! (FieldElement :: decompress (y , d) . 0 as i32 , ratdecompress (y as i32 , d . into ())) ; } } } # [test] fn test_compress_decompress_roundtrip_d_lt_12 () { for d in COMPRESSION_D { for x in 0 .. 2u32 . pow (d as u32) { assert_eq ! (FieldElement :: decompress (x , d) . compress (d) , x) ; } } } }
    };
}

test_compression!()