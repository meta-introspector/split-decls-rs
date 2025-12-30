// Generated macro for test (module)
macro_rules! Depcrate_data_floattest {
() => {
// Module: crate::data::float
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn test_pretty_printing () { assert_eq ! (pretty_print_float (0.99999999999999999999 , false) , "1") ; assert_eq ! (pretty_print_float (0.9999 , false) , "0.9999") ; assert_eq ! (pretty_print_float (- 1e-5 - 0.00000000000000001 , true) , "-1e-5") ; assert_eq ! (pretty_print_float (- 1e-5 - 0.00000000000000001 , false) , "-0.00001") ; assert_eq ! (pretty_print_float (1e100 , true) , "1e100") ; assert_eq ! (pretty_print_float (1234567890f64 , true) , "1234567890") ; assert_eq ! (pretty_print_float (1000000001f64 , true) , "1e9") ; } }
};
}
