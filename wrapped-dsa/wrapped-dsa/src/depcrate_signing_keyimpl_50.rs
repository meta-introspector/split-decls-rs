// Generated macro for impl_50 (impl)
macro_rules! Depcrate_signing_keyimpl_50 {
() => {
// Module: crate::signing_key
// Provides: {"impl_50"}
// Dependencies: {}
# [cfg (feature = "pkcs8")] impl < 'a > TryFrom < PrivateKeyInfoRef < 'a > > for SigningKey { type Error = pkcs8 :: Error ; fn try_from (value : PrivateKeyInfoRef < 'a >) -> Result < Self , Self :: Error > { value . algorithm . assert_algorithm_oid (OID) ? ; let parameters = value . algorithm . parameters_any () ? ; let components = parameters . decode_as :: < Components > () ? ; let precision = components . p () . bits_precision () ; let x = UintRef :: from_der (value . private_key . into ()) ? ; let x = BoxedUint :: from_be_slice (x . as_bytes () , precision) . map_err (| _ | pkcs8 :: Error :: KeyMalformed) ? ; let x = NonZero :: new (x) . into_option () . ok_or (pkcs8 :: Error :: KeyMalformed) ? ; let y = if let Some (y_bytes) = value . public_key . as_ref () . and_then (| bs | bs . as_bytes ()) { let y = UintRef :: from_der (y_bytes) ? ; BoxedUint :: from_be_slice (y . as_bytes () , precision) . map_err (| _ | pkcs8 :: Error :: KeyMalformed) ? } else { crate :: generate :: public_component (& components , & x) . into_option () . ok_or (pkcs8 :: Error :: KeyMalformed) ? . get () } ; let verifying_key = VerifyingKey :: from_components (components , y) . map_err (| _ | pkcs8 :: Error :: KeyMalformed) ? ; SigningKey :: from_components (verifying_key , x . get ()) . map_err (| _ | pkcs8 :: Error :: KeyMalformed) } }
};
}
