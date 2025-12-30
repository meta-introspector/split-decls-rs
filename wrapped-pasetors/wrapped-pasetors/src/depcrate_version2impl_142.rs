// Generated macro for impl_142 (impl)
macro_rules! Depcrate_version2impl_142 {
() => {
// Module: crate::version2
// Provides: {"impl_142"}
// Dependencies: {}
impl TryFrom < & AsymmetricSecretKey < V2 > > for AsymmetricPublicKey < V2 > { type Error = Error ; fn try_from (value : & AsymmetricSecretKey < V2 >) -> Result < Self , Self :: Error > { AsymmetricPublicKey :: < V2 > :: from (& value . as_bytes () [32 ..]) } }
};
}
