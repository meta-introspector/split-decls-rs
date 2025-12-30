// Generated macro for impl_23 (impl)
macro_rules! Depcrate_pkcs8impl_23 {
() => {
// Module: crate::pkcs8
// Provides: {"impl_23"}
// Dependencies: {}
impl TryFrom < PrivateKeyInfoRef < '_ > > for KeypairBytes { type Error = Error ; fn try_from (private_key : PrivateKeyInfoRef < '_ >) -> Result < Self > { private_key . algorithm . assert_algorithm_oid (ALGORITHM_OID) ? ; if private_key . algorithm . parameters . is_some () { return Err (Error :: ParametersMalformed) ; } let secret_key = match private_key . private_key . as_bytes () { [0x04 , 0x20 , rest @ ..] => rest . try_into () . map_err (| _ | Error :: KeyMalformed) , _ => Err (Error :: KeyMalformed) , } ? ; let public_key = private_key . public_key . and_then (| bs | bs . as_bytes ()) . map (| bytes | bytes . try_into () . map_err (| _ | Error :: KeyMalformed)) . transpose () ? . map (PublicKeyBytes) ; Ok (Self { secret_key , public_key , }) } }
};
}
