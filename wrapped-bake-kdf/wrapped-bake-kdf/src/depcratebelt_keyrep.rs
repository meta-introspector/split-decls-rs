// Generated macro for belt_keyrep (function)
macro_rules! Depcratebelt_keyrep {
() => {
// Module: crate
// Provides: {"belt_keyrep"}
// Dependencies: {}
# [doc = " `belt-keyrep` key repetition algorithm described in STB 34.101.34-2020 8.1.3."] # [doc = ""] # [doc = " # Panics"] # [doc = " If `(N, M)` is not equal to `(16, 16)`, `(24, 16)`, `(24, 24)`,"] # [doc = " `(32, 16)`, `(32, 24)`, or `(32, 32)`."] # [inline] pub fn belt_keyrep < const N : usize , const M : usize > (x : & [u8 ; N] , d : & [u8 ; 12] , i : & [u8 ; 16] ,) -> [u8 ; M] { let r : u32 = match (N , M) { (16 , 16) => 0xC8BA94B1 , (24 , 16) => 0x12D6E35B , (24 , 24) => 0xFFC0B05C , (32 , 16) => 0x1ADC2BE1 , (32 , 24) => 0x3876ABC1 , (32 , 32) => 0x7B653CF3 , _ => panic ! ("belt-keyrep: invalid combination of N ({N}) and M ({M})") , } ; let s = belt_keyexpand (x) ; let d = [u32 :: from_le_bytes (d [.. 4] . try_into () . unwrap ()) , u32 :: from_le_bytes (d [4 ..] [.. 4] . try_into () . unwrap ()) , u32 :: from_le_bytes (d [8 ..] [.. 4] . try_into () . unwrap ()) ,] ; let i = [u32 :: from_le_bytes (i [0 ..] [.. 4] . try_into () . unwrap ()) , u32 :: from_le_bytes (i [4 ..] [.. 4] . try_into () . unwrap ()) , u32 :: from_le_bytes (i [8 ..] [.. 4] . try_into () . unwrap ()) , u32 :: from_le_bytes (i [12 ..] [.. 4] . try_into () . unwrap ()) ,] ; let (_ , s) = belt_compress ([r , d [0] , d [1] , d [2]] , i , s) ; let mut y = [0u8 ; M] ; for (src , dst) in s . iter () . zip (y . chunks_exact_mut (4)) { dst . copy_from_slice (& src . to_le_bytes ()) ; } y }
};
}
