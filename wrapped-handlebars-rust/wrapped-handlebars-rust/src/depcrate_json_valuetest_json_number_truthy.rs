// Generated macro for test_json_number_truthy (function)
macro_rules! Depcrate_json_valuetest_json_number_truthy {
() => {
// Module: crate::json::value
// Provides: {"test_json_number_truthy"}
// Dependencies: {}
# [test] fn test_json_number_truthy () { use std :: f64 ; assert ! (json ! (16i16) . is_truthy (false)) ; assert ! (json ! (16i16) . is_truthy (true)) ; assert ! (json ! (0i16) . is_truthy (true)) ; assert ! (! json ! (0i16) . is_truthy (false)) ; assert ! (json ! (1.0f64) . is_truthy (false)) ; assert ! (json ! (1.0f64) . is_truthy (true)) ; assert ! (json ! (Some (16i16)) . is_truthy (false)) ; assert ! (json ! (Some (16i16)) . is_truthy (true)) ; assert ! (! json ! (None as Option < i16 >) . is_truthy (false)) ; assert ! (! json ! (None as Option < i16 >) . is_truthy (true)) ; assert ! (! json ! (f64 :: NAN) . is_truthy (false)) ; assert ! (! json ! (f64 :: NAN) . is_truthy (true)) ; }
};
}
