// Generated macro for impl_106 (impl)
macro_rules! Depcrate_ecimpl_106 {
() => {
// Module: crate::ec
// Provides: {"impl_106"}
// Dependencies: {}
impl Curve for P384 { fn group (_ : sealed :: Sealed) -> Group { Group :: P384 } fn hash (data : & [u8]) -> Vec < u8 > { crate :: digest :: Sha384 :: hash (data) . to_vec () } }
};
}
