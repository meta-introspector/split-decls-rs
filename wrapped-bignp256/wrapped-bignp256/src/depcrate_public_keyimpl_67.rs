// Generated macro for impl_67 (impl)
macro_rules! Depcrate_public_keyimpl_67 {
() => {
// Module: crate::public_key
// Provides: {"impl_67"}
// Dependencies: {}
# [cfg (feature = "pkcs8")] impl TryFrom < & pkcs8 :: SubjectPublicKeyInfoRef < '_ > > for PublicKey { type Error = pkcs8 :: spki :: Error ; fn try_from (spki : & pkcs8 :: SubjectPublicKeyInfoRef < '_ >) -> pkcs8 :: spki :: Result < Self > { spki . algorithm . assert_oids (ALGORITHM_OID , BignP256 :: OID) ? ; let public_key_bytes = spki . subject_public_key . as_bytes () . ok_or_else (| | der :: Tag :: BitString . value_error () . to_error ()) ? ; Self :: from_bytes (public_key_bytes) . map_err (| _ | pkcs8 :: spki :: Error :: KeyMalformed) } }
};
}
