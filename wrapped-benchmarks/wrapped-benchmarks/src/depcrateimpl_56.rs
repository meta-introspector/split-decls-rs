// Generated macro for impl_56 (impl)
macro_rules! Depcrateimpl_56 {
() => {
// Module: crate
// Provides: {"impl_56"}
// Dependencies: {}
impl Generate for FunctionCallAction { fn generate < R : rand :: Rng > (rng : & mut R) -> Self { Self { method_name : String :: generate (rng) , args : generate_vec_u8 (rng , 0 , 1000) , gas : u64 :: generate (rng) , deposit : u64 :: generate (rng) , } } }
};
}
