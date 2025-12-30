// Generated macro for tests (module)
macro_rules! Depcrate_public_keytests {
() => {
// Module: crate::public_key
// Provides: {"tests"}
// Dependencies: {}
# [cfg (all (feature = "dev" , test))] mod tests { use crate :: { dev :: MockCurve , sec1 :: FromEncodedPoint } ; type EncodedPoint = crate :: sec1 :: EncodedPoint < MockCurve > ; type PublicKey = super :: PublicKey < MockCurve > ; # [test] fn from_encoded_point_rejects_identity () { let identity = EncodedPoint :: identity () ; assert ! (bool :: from (PublicKey :: from_encoded_point (& identity) . is_none ())) ; } }
};
}
