// Generated macro for impl_208 (impl)
macro_rules! Depcrate_distr_uniform_otherimpl_208 {
() => {
// Module: crate::distr::uniform::other
// Provides: {"impl_208"}
// Dependencies: {}
impl Distribution < u8 > for Alphabetic { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> u8 { const RANGE : u8 = 26 + 26 ; let offset = rng . random_range (0 .. RANGE) + b'A' ; offset + (offset > b'Z') as u8 * (b'a' - b'Z' - 1) } }
};
}
