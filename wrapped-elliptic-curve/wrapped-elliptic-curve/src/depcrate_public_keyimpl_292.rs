// Generated macro for impl_292 (impl)
macro_rules! Depcrate_public_keyimpl_292 {
() => {
// Module: crate::public_key
// Provides: {"impl_292"}
// Dependencies: {}
# [cfg (feature = "pkcs8")] impl < C > TryFrom < pkcs8 :: SubjectPublicKeyInfoRef < '_ > > for PublicKey < C > where C : AssociatedOid + CurveArithmetic , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : ModulusSize , { type Error = pkcs8 :: spki :: Error ; fn try_from (spki : pkcs8 :: SubjectPublicKeyInfoRef < '_ >) -> pkcs8 :: spki :: Result < Self > { Self :: try_from (& spki) } }
};
}
