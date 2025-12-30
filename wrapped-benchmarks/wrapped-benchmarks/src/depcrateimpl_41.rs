// Generated macro for impl_41 (impl)
macro_rules! Depcrateimpl_41 {
() => {
// Module: crate
// Provides: {"impl_41"}
// Dependencies: {}
impl Generate for Block { fn generate < R : rand :: Rng > (rng : & mut R) -> Self { Self { header : BlockHeader :: generate (rng) , transactions : generate_vec (rng , 0 , 1000) , } } }
};
}
