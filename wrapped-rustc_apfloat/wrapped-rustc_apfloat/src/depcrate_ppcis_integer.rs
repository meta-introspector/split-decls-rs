// Generated macro for is_integer (function)
macro_rules! Depcrate_ppcis_integer {
() => {
// Module: crate::ppc
// Provides: {"is_integer"}
// Dependencies: {}
# [test] fn is_integer () { let double_from_f64 = | f : f64 | ieee :: Double :: from_bits (f . to_bits () . into ()) ; assert ! (DoubleFloat (double_from_f64 (- 0.0) , double_from_f64 (- 0.0)) . is_integer ()) ; assert ! (! DoubleFloat (double_from_f64 (3.14159) , double_from_f64 (- 0.0)) . is_integer ()) ; assert ! (! DoubleFloat (double_from_f64 (- 0.0) , double_from_f64 (3.14159)) . is_integer ()) ; }
};
}
