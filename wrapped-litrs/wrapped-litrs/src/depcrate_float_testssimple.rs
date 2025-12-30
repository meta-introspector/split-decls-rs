// Generated macro for simple (function)
macro_rules! Depcrate_float_testssimple {
() => {
// Module: crate::float::tests
// Provides: {"simple"}
// Dependencies: {}
# [test] fn simple () { check ! ("3" ".14" "" -) ; check ! ("3" ".14" "" f32) ; check ! ("3" ".14" "" f64) ; check ! ("3" "" "e987654321" -) ; check ! ("3" "" "e987654321" f64) ; check ! ("42_888" ".05" "" -) ; check ! ("42_888" ".05" "E5___" f32) ; check ! ("123456789" "" "e_1" f64) ; check ! ("123456789" ".99" "e_1" f64) ; check ! ("123456789" ".99" "" f64) ; check ! ("123456789" ".99" "" -) ; check ! ("147" ".3_33" "" -) ; check ! ("147" ".3_33__" "E3" f64) ; check ! ("147" ".3_33__" "" f32) ; check ! ("147" ".333" "e-10" -) ; check ! ("147" ".333" "e-_7" f32) ; check ! ("147" ".333" "e+10" -) ; check ! ("147" ".333" "e+_7" f32) ; check ! ("86" "." "" -) ; check ! ("0" "." "" -) ; check ! ("0_" "." "" -) ; check ! ("0" ".0000001" "" -) ; check ! ("0" ".000_0001" "" -) ; check ! ("0" ".0" "e+0" -) ; check ! ("0" "" "E+0" -) ; check ! ("34" "" "e+0" -) ; check ! ("0" ".9182" "E+0" f32) ; }
};
}
