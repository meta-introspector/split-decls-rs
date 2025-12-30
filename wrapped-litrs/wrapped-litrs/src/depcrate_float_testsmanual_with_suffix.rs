// Generated macro for manual_with_suffix (function)
macro_rules! Depcrate_float_testsmanual_with_suffix {
() => {
// Module: crate::float::tests
// Provides: {"manual_with_suffix"}
// Dependencies: {}
# [test] fn manual_with_suffix () -> Result < () , ParseError > { let f = FloatLit :: parse ("3.14f32") ? ; assert_eq ! (f . number_part () , "3.14") ; assert_eq ! (f . integer_part () , "3") ; assert_eq ! (f . fractional_part () , Some ("14")) ; assert_eq ! (f . exponent_part () , "") ; assert_eq ! (FloatType :: from_suffix (f . suffix ()) , Some (FloatType :: F32)) ; let f = FloatLit :: parse ("8e1f64") ? ; assert_eq ! (f . number_part () , "8e1") ; assert_eq ! (f . integer_part () , "8") ; assert_eq ! (f . fractional_part () , None) ; assert_eq ! (f . exponent_part () , "e1") ; assert_eq ! (FloatType :: from_suffix (f . suffix ()) , Some (FloatType :: F64)) ; let f = FloatLit :: parse ("8_7_6.1_23e15f32") ? ; assert_eq ! (f . number_part () , "8_7_6.1_23e15") ; assert_eq ! (f . integer_part () , "8_7_6") ; assert_eq ! (f . fractional_part () , Some ("1_23")) ; assert_eq ! (f . exponent_part () , "e15") ; assert_eq ! (FloatType :: from_suffix (f . suffix ()) , Some (FloatType :: F32)) ; let f = FloatLit :: parse ("8.2e-_04_9f64") ? ; assert_eq ! (f . number_part () , "8.2e-_04_9") ; assert_eq ! (f . integer_part () , "8") ; assert_eq ! (f . fractional_part () , Some ("2")) ; assert_eq ! (f . exponent_part () , "e-_04_9") ; assert_eq ! (FloatType :: from_suffix (f . suffix ()) , Some (FloatType :: F64)) ; Ok (()) }
};
}
