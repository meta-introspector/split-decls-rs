// Generated macro for P384_DOMAIN_NO_SEED (const)
macro_rules! Depcrate_ec_constantsP384_DOMAIN_NO_SEED {
() => {
// Module: crate::ec_constants
// Provides: {"P384_DOMAIN_NO_SEED"}
// Dependencies: {}
pub const P384_DOMAIN_NO_SEED : SpecifiedECDomain < 'static > = SpecifiedECDomain { version : 1 , field_id : P384_FIELD , curve : Curve { a : P384_CURVE_A , b : P384_CURVE_B , seed : None , } , base : P384_UNCOMPRESSED_BASE , order : P384_ORDER , cofactor : Some (1) , } ;
};
}
