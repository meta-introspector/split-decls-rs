// Generated macro for impl_68 (impl)
macro_rules! Depcrateimpl_68 {
() => {
// Module: crate
// Provides: {"impl_68"}
// Dependencies: {}
impl Generate for AccessKey { fn generate < R : rand :: Rng > (rng : & mut R) -> Self { Self { nonce : u64 :: generate (rng) , permission : AccessKeyPermission :: generate (rng) , } } }
};
}
