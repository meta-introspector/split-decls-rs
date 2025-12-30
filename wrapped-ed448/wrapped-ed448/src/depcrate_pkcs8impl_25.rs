// Generated macro for impl_25 (impl)
macro_rules! Depcrate_pkcs8impl_25 {
() => {
// Module: crate::pkcs8
// Provides: {"impl_25"}
// Dependencies: {}
impl TryFrom < spki :: SubjectPublicKeyInfoRef < '_ > > for PublicKeyBytes { type Error = spki :: Error ; fn try_from (spki : spki :: SubjectPublicKeyInfoRef < '_ >) -> spki :: Result < Self > { spki . algorithm . assert_algorithm_oid (ALGORITHM_OID) ? ; if spki . algorithm . parameters . is_some () { return Err (spki :: Error :: KeyMalformed) ; } spki . subject_public_key . as_bytes () . ok_or (spki :: Error :: KeyMalformed) ? . try_into () . map (Self) . map_err (| _ | spki :: Error :: KeyMalformed) } }
};
}
