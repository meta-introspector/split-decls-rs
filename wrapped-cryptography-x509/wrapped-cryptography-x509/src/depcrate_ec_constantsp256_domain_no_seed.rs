// Generated macro for P256_DOMAIN_NO_SEED (const)
macro_rules! Depcrate_ec_constantsP256_DOMAIN_NO_SEED {
() => {
// Module: crate::ec_constants
// Provides: {"P256_DOMAIN_NO_SEED"}
// Dependencies: {}
pub const P256_DOMAIN_NO_SEED : SpecifiedECDomain < 'static > = SpecifiedECDomain { version : 1 , field_id : P256_FIELD , curve : Curve { a : P256_CURVE_A , b : P256_CURVE_B , seed : None , } , base : P256_UNCOMPRESSED_BASE , order : P256_ORDER , cofactor : Some (1) , } ;
};
}
