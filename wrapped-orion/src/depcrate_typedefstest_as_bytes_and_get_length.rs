// Generated macro for test_as_bytes_and_get_length (macro)
macro_rules! Depcrate_typedefstest_as_bytes_and_get_length {
() => {
// Module: crate::typedefs
// Provides: {"test_as_bytes_and_get_length"}
// Dependencies: {}
# [cfg (test)] macro_rules ! test_as_bytes_and_get_length (($ name : ident , $ lower_bound : expr , $ upper_bound : expr , $ bytes_function : ident) => (# [test] fn test_as_bytes () { let test_upper = $ name :: from_slice (& [0u8 ; $ upper_bound]) . unwrap () ; let test_lower = $ name :: from_slice (& [0u8 ; $ lower_bound]) . unwrap () ; assert_eq ! (test_upper .$ bytes_function () . len () , test_upper . len ()) ; assert_eq ! (test_upper . len () , $ upper_bound) ; assert_eq ! (test_lower .$ bytes_function () . len () , test_lower . len ()) ; assert_eq ! (test_lower . len () , $ lower_bound) ; assert_eq ! (test_upper . is_empty () , false) ; assert_eq ! (test_lower . is_empty () , false) ; if $ lower_bound != $ upper_bound { let test_upper = $ name :: from_slice (& [0u8 ; $ upper_bound - 1]) . unwrap () ; let test_lower = $ name :: from_slice (& [0u8 ; $ lower_bound + 1]) . unwrap () ; assert_eq ! (test_upper .$ bytes_function () . len () , test_upper . len ()) ; assert_eq ! (test_upper . len () , $ upper_bound - 1) ; assert_eq ! (test_lower .$ bytes_function () . len () , test_lower . len ()) ; assert_eq ! (test_lower . len () , $ lower_bound + 1) ; assert_eq ! (test_upper . is_empty () , false) ; assert_eq ! (test_lower . is_empty () , false) ; } })) ;
};
}
