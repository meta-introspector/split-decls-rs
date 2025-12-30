// Generated macro for impl_61 (impl)
macro_rules! Depcrate_pbes2impl_61 {
() => {
// Module: crate::pbes2
// Provides: {"impl_61"}
// Dependencies: {}
impl TryFrom < AnyRef < '_ > > for Parameters { type Error = der :: Error ; fn try_from (any : AnyRef < '_ >) -> der :: Result < Self > { any . sequence (| params | { let kdf = AlgorithmIdentifierRef :: decode (params) ? ; let encryption = AlgorithmIdentifierRef :: decode (params) ? ; Ok (Self { kdf : kdf . try_into () ? , encryption : encryption . try_into () ? , }) }) } }
};
}
