// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
impl KeyIvInit for Hc256Core { fn new (key : & Key < Self > , iv : & Iv < Self >) -> Self { fn f1 (x : u32) -> u32 { x . rotate_right (7) ^ x . rotate_right (18) ^ (x >> 3) } fn f2 (x : u32) -> u32 { x . rotate_right (17) ^ x . rotate_right (19) ^ (x >> 10) } let mut out = Self { ptable : [0 ; TABLE_SIZE] , qtable : [0 ; TABLE_SIZE] , idx : 0 , } ; let mut data = [0 ; INIT_SIZE] ; for i in 0 .. KEY_WORDS { data [i] = key [4 * i] as u32 & 0xff | ((key [(4 * i) + 1] as u32 & 0xff) << 8) | ((key [(4 * i) + 2] as u32 & 0xff) << 16) | ((key [(4 * i) + 3] as u32 & 0xff) << 24) ; } for i in 0 .. IV_WORDS { data [i + KEY_WORDS] = iv [4 * i] as u32 & 0xff | ((iv [(4 * i) + 1] as u32 & 0xff) << 8) | ((iv [(4 * i) + 2] as u32 & 0xff) << 16) | ((iv [(4 * i) + 3] as u32 & 0xff) << 24) ; } for i in IV_WORDS + KEY_WORDS .. INIT_SIZE { data [i] = f2 (data [i - 2]) . wrapping_add (data [i - 7]) . wrapping_add (f1 (data [i - 15])) . wrapping_add (data [i - 16]) . wrapping_add (i as u32) ; } out . ptable [.. TABLE_SIZE] . clone_from_slice (& data [512 .. (TABLE_SIZE + 512)]) ; out . qtable [.. TABLE_SIZE] . clone_from_slice (& data [1536 .. (TABLE_SIZE + 1536)]) ; out . idx = 0 ; for _ in 0 .. 4096 { out . gen_word () ; } out } }
};
}
