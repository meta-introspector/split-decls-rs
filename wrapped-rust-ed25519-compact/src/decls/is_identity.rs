macro_rules! is_identity {
    () => {
        pub fn is_identity (s : & [u8 ; 32]) -> bool { let mut c = s [0] ^ 0x01 ; for i in 1 .. 31 { c |= s [i] ; } c |= s [31] & 0x7f ; c == 0 }
    };
}

is_identity!();