// Generated macro for add_poly (function)
macro_rules! Depcrate_hazardous_kem_ml_kem_internal_readd_poly {
() => {
// Module: crate::hazardous::kem::ml_kem::internal::re
// Provides: {"add_poly"}
// Dependencies: {}
fn add_poly (p1 : & [FieldElement ; 256] , p2 : & [FieldElement ; 256] , ret : & mut [FieldElement ; 256]) { for idx in 0 .. 256 { ret [idx] = p1 [idx] + p2 [idx] ; } }
};
}
