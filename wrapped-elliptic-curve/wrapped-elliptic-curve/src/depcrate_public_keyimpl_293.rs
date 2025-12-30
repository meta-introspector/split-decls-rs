// Generated macro for impl_293 (impl)
macro_rules! Depcrate_public_keyimpl_293 {
() => {
// Module: crate::public_key
// Provides: {"impl_293"}
// Dependencies: {}
# [cfg (feature = "pkcs8")] impl < C > TryFrom < & pkcs8 :: SubjectPublicKeyInfoRef < '_ > > for PublicKey < C > where C : AssociatedOid + CurveArithmetic , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : ModulusSize , { type Error = pkcs8 :: spki :: Error ; fn try_from (spki : & pkcs8 :: SubjectPublicKeyInfoRef < '_ >) -> pkcs8 :: spki :: Result < Self > { spki . algorithm . assert_oids (ALGORITHM_OID , C :: OID) ? ; let public_key_bytes = spki . subject_public_key . as_bytes () . ok_or_else (| | der :: Tag :: BitString . value_error () . to_error ()) ? ; Self :: from_sec1_bytes (public_key_bytes) . map_err (| _ | der :: Tag :: BitString . value_error () . to_error () . into ()) } }
};
}
