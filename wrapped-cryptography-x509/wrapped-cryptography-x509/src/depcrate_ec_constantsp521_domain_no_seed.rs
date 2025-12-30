// Generated macro for P521_DOMAIN_NO_SEED (const)
macro_rules! Depcrate_ec_constantsP521_DOMAIN_NO_SEED {
() => {
// Module: crate::ec_constants
// Provides: {"P521_DOMAIN_NO_SEED"}
// Dependencies: {}
pub const P521_DOMAIN_NO_SEED : SpecifiedECDomain < 'static > = SpecifiedECDomain { version : 1 , field_id : P521_FIELD , curve : Curve { a : P521_CURVE_A , b : P521_CURVE_B , seed : None , } , base : P521_UNCOMPRESSED_BASE , order : P521_ORDER , cofactor : Some (1) , } ;
};
}
