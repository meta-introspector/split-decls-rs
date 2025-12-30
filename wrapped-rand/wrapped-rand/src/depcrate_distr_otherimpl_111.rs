// Generated macro for impl_111 (impl)
macro_rules! Depcrate_distr_otherimpl_111 {
() => {
// Module: crate::distr::other
// Provides: {"impl_111"}
// Dependencies: {}
impl Distribution < u8 > for Alphanumeric { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> u8 { const RANGE : u32 = 26 + 26 + 10 ; const GEN_ASCII_STR_CHARSET : & [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                abcdefghijklmnopqrstuvwxyz\
                0123456789" ; loop { let var = rng . next_u32 () >> (32 - 6) ; if var < RANGE { return GEN_ASCII_STR_CHARSET [var as usize] ; } } } }
};
}
