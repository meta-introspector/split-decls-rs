// Generated macro for encrypt (function)
macro_rules! Depcrate_block_apiencrypt {
() => {
// Module: crate::block_api
// Provides: {"encrypt"}
// Dependencies: {}
# [allow (clippy :: needless_range_loop)] fn encrypt (msg : & mut [u8] , key : Block , sbox : & SBox) { let mut k = [0u32 ; 8] ; let mut a = u32 :: from_le_bytes (msg [0 .. 4] . try_into () . unwrap ()) ; let mut b = u32 :: from_le_bytes (msg [4 .. 8] . try_into () . unwrap ()) ; for (o , chunk) in k . iter_mut () . zip (key . chunks_exact (4)) { * o = u32 :: from_le_bytes (chunk . try_into () . unwrap ()) ; } for _ in 0 .. 3 { for i in 0 .. 8 { let t = b ^ g (a , k [i] , sbox) ; b = a ; a = t ; } } for i in (0 .. 8) . rev () { let t = b ^ g (a , k [i] , sbox) ; b = a ; a = t ; } msg [0 .. 4] . copy_from_slice (& b . to_le_bytes ()) ; msg [4 .. 8] . copy_from_slice (& a . to_le_bytes ()) ; }
};
}
