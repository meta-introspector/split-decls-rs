// Generated macro for impl_104 (impl)
macro_rules! Depcrate_ecimpl_104 {
() => {
// Module: crate::ec
// Provides: {"impl_104"}
// Dependencies: {}
impl Curve for P256 { fn group (_ : sealed :: Sealed) -> Group { Group :: P256 } fn hash (data : & [u8]) -> Vec < u8 > { crate :: digest :: Sha256 :: hash (data) . to_vec () } }
};
}
