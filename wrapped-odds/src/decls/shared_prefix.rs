macro_rules! shared_prefix {
    () => {
        # [doc = " Return the end index of the longest shared (equal) prefix of `a` and `b`."] pub fn shared_prefix (a : & [u8] , b : & [u8]) -> usize { let len = min (a . len () , b . len ()) ; let mut a = & a [.. len] ; let mut b = & b [.. len] ; let mut offset = 0 ; while a . len () >= 16 { unsafe { let a0 = load_u64 (a , 0) ; let a1 = load_u64 (a , 8) ; let b0 = load_u64 (b , 0) ; let b1 = load_u64 (b , 8) ; let d0 = a0 ^ b0 ; let d1 = a1 ^ b1 ; if d0 ^ d1 != 0 { break ; } } offset += 16 ; a = & a [16 ..] ; b = & b [16 ..] ; } for i in 0 .. a . len () { if a [i] != b [i] { return offset + i ; } } len }
    };
}

shared_prefix!();