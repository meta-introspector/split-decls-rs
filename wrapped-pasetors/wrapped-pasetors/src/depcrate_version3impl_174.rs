// Generated macro for impl_174 (impl)
macro_rules! Depcrate_version3impl_174 {
() => {
// Module: crate::version3
// Provides: {"impl_174"}
// Dependencies: {}
impl TryFrom < & AsymmetricPublicKey < V3 > > for UncompressedPublicKey { type Error = Error ; fn try_from (value : & AsymmetricPublicKey < V3 >) -> Result < Self , Self :: Error > { if value . as_bytes () [0] != 2 && value . as_bytes () [0] != 3 { return Err (Error :: Key) ; } let pk = PublicKey :: from_sec1_bytes (value . as_bytes ()) . map_err (| _ | Error :: Key) ? ; Ok (UncompressedPublicKey (pk)) } }
};
}
