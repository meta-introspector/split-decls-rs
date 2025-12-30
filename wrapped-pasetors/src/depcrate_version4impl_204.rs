// Generated macro for impl_204 (impl)
macro_rules! Depcrate_version4impl_204 {
() => {
// Module: crate::version4
// Provides: {"impl_204"}
// Dependencies: {}
impl TryFrom < & AsymmetricSecretKey < V4 > > for AsymmetricPublicKey < V4 > { type Error = Error ; fn try_from (value : & AsymmetricSecretKey < V4 >) -> Result < Self , Self :: Error > { AsymmetricPublicKey :: < V4 > :: from (& value . as_bytes () [32 ..]) } }
};
}
