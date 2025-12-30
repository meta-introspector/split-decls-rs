// Generated macro for test_any_number_precision (function)
macro_rules! Depcrate_ser_teststest_any_number_precision {
() => {
// Module: crate::ser::tests
// Provides: {"test_any_number_precision"}
// Dependencies: {}
# [test] fn test_any_number_precision () { check_ser_any_number (1_u8) ; check_ser_any_number (- 1_i8) ; check_ser_any_number (1_f32) ; check_ser_any_number (- 1_f32) ; check_ser_any_number (0.3_f64) ; check_to_string_writer (& Number :: new (f32 :: NAN) , "NaN" , "NaN") ; check_to_string_writer (& f32 :: NAN , "NaN" , "NaN") ; check_to_string_writer (& Number :: new (- f32 :: NAN) , "-NaN" , "-NaN") ; check_to_string_writer (& (- f32 :: NAN) , "-NaN" , "-NaN") ; check_to_string_writer (& Number :: new (f32 :: INFINITY) , "inf" , "inf") ; check_to_string_writer (& f32 :: INFINITY , "inf" , "inf") ; check_to_string_writer (& Number :: new (f32 :: NEG_INFINITY) , "-inf" , "-inf") ; check_to_string_writer (& f32 :: NEG_INFINITY , "-inf" , "-inf") ; macro_rules ! test_min_max { ($ ty : ty) => { check_ser_any_number (<$ ty >:: MIN) ; check_ser_any_number (<$ ty >:: MAX) ; } ; ($ ($ ty : ty) ,*) => { $ (test_min_max ! { $ ty }) * } ; } test_min_max ! { i8 , i16 , i32 , i64 , u8 , u16 , u32 , u64 , f32 , f64 } # [cfg (feature = "integer128")] test_min_max ! { i128 , u128 } }
};
}
