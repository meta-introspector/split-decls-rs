// Generated macro for test_any_number_precision (function)
macro_rules! Depcrate_de_teststest_any_number_precision {
() => {
// Module: crate::de::tests
// Provides: {"test_any_number_precision"}
// Dependencies: {}
# [test] fn test_any_number_precision () { check_de_any_number ("1" , 1_u8) ; check_de_any_number ("+1" , 1_u8) ; check_de_any_number ("-1" , - 1_i8) ; check_de_any_number ("-1.0" , - 1.0_f32) ; check_de_any_number ("1." , 1.0_f32) ; check_de_any_number ("-1." , - 1.0_f32) ; check_de_any_number (".3" , 0.3_f64) ; check_de_any_number ("-.3" , - 0.3_f64) ; check_de_any_number ("+.3" , 0.3_f64) ; check_de_any_number ("0.3" , 0.3_f64) ; check_de_any_number ("NaN" , f32 :: NAN) ; check_de_any_number ("-NaN" , - f32 :: NAN) ; check_de_any_number ("inf" , f32 :: INFINITY) ; check_de_any_number ("-inf" , f32 :: NEG_INFINITY) ; macro_rules ! test_min { ($ ($ ty : ty) ,*) => { $ (check_de_any_number (& format ! ("{}" , <$ ty >:: MIN) , <$ ty >:: MIN) ;) * } ; } macro_rules ! test_max { ($ ($ ty : ty) ,*) => { $ (check_de_any_number (& format ! ("{}" , <$ ty >:: MAX) , <$ ty >:: MAX) ;) * } ; } test_min ! { i8 , i16 , i32 , i64 , f64 } test_max ! { u8 , u16 , u32 , u64 , f64 } # [cfg (feature = "integer128")] test_min ! { i128 } # [cfg (feature = "integer128")] test_max ! { u128 } }
};
}
