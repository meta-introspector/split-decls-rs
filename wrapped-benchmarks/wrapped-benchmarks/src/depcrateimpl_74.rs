// Generated macro for impl_74 (impl)
macro_rules! Depcrateimpl_74 {
() => {
// Module: crate
// Provides: {"impl_74"}
// Dependencies: {}
impl Generate for FunctionCallPermission { fn generate < R : rand :: Rng > (rng : & mut R) -> Self { Self { allowance : generate_option (rng) , receiver_id : AccountId :: generate (rng) , method_names : generate_vec (rng , 0 , 10) , } } }
};
}
