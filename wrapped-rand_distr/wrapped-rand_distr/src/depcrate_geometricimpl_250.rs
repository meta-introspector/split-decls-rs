// Generated macro for impl_250 (impl)
macro_rules! Depcrate_geometricimpl_250 {
() => {
// Module: crate::geometric
// Provides: {"impl_250"}
// Dependencies: {}
impl Distribution < u64 > for StandardGeometric { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> u64 { let mut result = 0 ; loop { let x = rng . random :: < u64 > () . leading_zeros () as u64 ; result += x ; if x < 64 { break ; } } result } }
};
}
