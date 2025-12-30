// Generated macro for P384_DOMAIN (const)
macro_rules! Depcrate_ec_constantsP384_DOMAIN {
() => {
// Module: crate::ec_constants
// Provides: {"P384_DOMAIN"}
// Dependencies: {}
pub const P384_DOMAIN : SpecifiedECDomain < 'static > = SpecifiedECDomain { version : 1 , field_id : P384_FIELD , curve : Curve { a : P384_CURVE_A , b : P384_CURVE_B , seed : Some (P384_SEED) , } , base : P384_UNCOMPRESSED_BASE , order : P384_ORDER , cofactor : Some (1) , } ;
};
}
