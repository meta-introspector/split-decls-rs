// Generated macro for impl_70 (impl)
macro_rules! Depcrateimpl_70 {
() => {
// Module: crate
// Provides: {"impl_70"}
// Dependencies: {}
impl Generate for AccessKeyPermission { fn generate < R : rand :: Rng > (rng : & mut R) -> Self { if u64 :: generate (rng) % 2 == 0 { AccessKeyPermission :: FunctionCall (FunctionCallPermission :: generate (rng)) } else { AccessKeyPermission :: FullAccess } } }
};
}
