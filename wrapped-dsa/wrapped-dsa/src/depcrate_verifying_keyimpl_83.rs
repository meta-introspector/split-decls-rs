// Generated macro for impl_83 (impl)
macro_rules! Depcrate_verifying_keyimpl_83 {
() => {
// Module: crate::verifying_key
// Provides: {"impl_83"}
// Dependencies: {}
# [cfg (feature = "pkcs8")] impl < 'a > TryFrom < SubjectPublicKeyInfoRef < 'a > > for VerifyingKey { type Error = spki :: Error ; fn try_from (value : SubjectPublicKeyInfoRef < 'a >) -> Result < Self , Self :: Error > { value . algorithm . assert_algorithm_oid (OID) ? ; let parameters = value . algorithm . parameters_any () ? ; let components = parameters . decode_as () ? ; let y = UintRef :: from_der (value . subject_public_key . as_bytes () . ok_or (spki :: Error :: KeyMalformed) ? ,) ? ; let y = BoxedUint :: from_be_slice_vartime (y . as_bytes ()) ; Self :: from_components (components , y) . map_err (| _ | spki :: Error :: KeyMalformed) } }
};
}
