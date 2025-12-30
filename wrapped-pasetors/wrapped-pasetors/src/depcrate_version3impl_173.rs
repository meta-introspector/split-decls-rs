// Generated macro for impl_173 (impl)
macro_rules! Depcrate_version3impl_173 {
() => {
// Module: crate::version3
// Provides: {"impl_173"}
// Dependencies: {}
impl TryFrom < & [u8] > for UncompressedPublicKey { type Error = Error ; fn try_from (value : & [u8]) -> Result < Self , Self :: Error > { if value . len () != 97 && value [0] != 4 { return Err (Error :: Key) ; } let pk = PublicKey :: from_sec1_bytes (value) . map_err (| _ | Error :: Key) ? ; Ok (Self (pk)) } }
};
}
