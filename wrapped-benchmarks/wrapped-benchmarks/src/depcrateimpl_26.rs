// Generated macro for impl_26 (impl)
macro_rules! Depcrateimpl_26 {
() => {
// Module: crate
// Provides: {"impl_26"}
// Dependencies: {}
impl Generate for String { fn generate < R : rand :: Rng > (rng : & mut R) -> Self { let len : usize = rng . gen_range (5 , 200) ; rng . sample_iter (& Alphanumeric) . take (len) . collect :: < String > () } }
};
}
