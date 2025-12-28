macro_rules! hex_decode_fallback {
    () => {
        pub fn hex_decode_fallback (src : & [u8] , dst : & mut [u8]) { for (slot , bytes) in dst . iter_mut () . zip (src . chunks_exact (2)) { let a = unhex_a (bytes [0] as usize) ; let b = unhex_b (bytes [1] as usize) ; * slot = a | b ; } }
    };
}

hex_decode_fallback!()