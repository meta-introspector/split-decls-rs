// Generated macro for impl_249 (impl)
macro_rules! Depcrate_secret_keyimpl_249 {
() => {
// Module: crate::secret_key
// Provides: {"impl_249"}
// Dependencies: {}
# [cfg (feature = "sec1")] impl < C > TryFrom < sec1 :: EcPrivateKey < '_ > > for SecretKey < C > where C : AssociatedOid + Curve + ValidatePublicKey , FieldBytesSize < C > : ModulusSize , { type Error = der :: Error ; fn try_from (sec1_private_key : sec1 :: EcPrivateKey < '_ >) -> der :: Result < Self > { if let Some (sec1 :: EcParameters :: NamedCurve (curve_oid)) = sec1_private_key . parameters { if C :: OID != curve_oid { return Err (der :: Tag :: ObjectIdentifier . value_error () . into ()) ; } } let secret_key = Self :: from_slice (sec1_private_key . private_key) . map_err (| _ | der :: Tag :: OctetString . value_error ()) ? ; if let Some (pk_bytes) = sec1_private_key . public_key { let pk = EncodedPoint :: < C > :: from_bytes (pk_bytes) . map_err (| _ | der :: Tag :: BitString . value_error ()) ? ; if C :: validate_public_key (& secret_key , & pk) . is_err () { return Err (der :: Tag :: BitString . value_error () . into ()) ; } } Ok (secret_key) } }
};
}
