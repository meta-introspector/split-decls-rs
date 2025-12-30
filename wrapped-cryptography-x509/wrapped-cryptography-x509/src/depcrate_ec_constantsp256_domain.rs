// Generated macro for P256_DOMAIN (const)
macro_rules! Depcrate_ec_constantsP256_DOMAIN {
() => {
// Module: crate::ec_constants
// Provides: {"P256_DOMAIN"}
// Dependencies: {}
pub const P256_DOMAIN : SpecifiedECDomain < 'static > = SpecifiedECDomain { version : 1 , field_id : P256_FIELD , curve : Curve { a : P256_CURVE_A , b : P256_CURVE_B , seed : Some (P256_SEED) , } , base : P256_UNCOMPRESSED_BASE , order : P256_ORDER , cofactor : Some (1) , } ;
};
}
