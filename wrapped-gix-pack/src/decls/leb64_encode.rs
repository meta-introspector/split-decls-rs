macro_rules! leb64_encode {
    () => {
        # [inline] fn leb64_encode (mut n : u64 , buf : & mut [u8 ; 10]) -> & [u8] { let mut bytes_written = 1 ; buf [buf . len () - 1] = n as u8 & 0b0111_1111 ; for out in buf . iter_mut () . rev () . skip (1) { n >>= 7 ; if n == 0 { break ; } n -= 1 ; * out = 0b1000_0000 | (n as u8 & 0b0111_1111) ; bytes_written += 1 ; } debug_assert_eq ! (n , 0 , "BUG: buffer must be large enough to hold a 64 bit integer") ; & buf [buf . len () - bytes_written ..] }
    };
}

leb64_encode!()