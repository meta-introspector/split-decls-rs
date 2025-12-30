// Generated macro for belt_keyexpand (function)
macro_rules! Depcratebelt_keyexpand {
() => {
// Module: crate
// Provides: {"belt_keyexpand"}
// Dependencies: {}
# [doc = " `belt-keyexpand` key expansion algorithm described in STB 34.101.34-2020 8.1.2."] # [doc = ""] # [doc = " # Panics"] # [doc = " If `N` is not equal to 16, 24, or 32."] # [inline] pub fn belt_keyexpand < const N : usize > (k : & [u8 ; N]) -> [u32 ; 8] { let mut t = [0u32 ; 8] ; for (src , dst) in k . chunks_exact (4) . zip (t . iter_mut ()) { * dst = u32 :: from_le_bytes (src . try_into () . unwrap ()) ; } match N { 16 => { t [4] = t [0] ; t [5] = t [1] ; t [6] = t [2] ; t [7] = t [3] ; } 24 => { t [6] = t [0] ^ t [1] ^ t [2] ; t [7] = t [3] ^ t [4] ^ t [5] ; } 32 => { } _ => panic ! ("Invalid key size n={N}. Expected 16, 24, or 32.") , } t }
};
}
