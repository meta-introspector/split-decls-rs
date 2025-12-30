// Generated macro for test_value_special_floats (function)
macro_rules! Depcrate_de_teststest_value_special_floats {
() => {
// Module: crate::de::tests
// Provides: {"test_value_special_floats"}
// Dependencies: {}
# [test] fn test_value_special_floats () { use crate :: { from_str , value :: Number , Value } ; assert_eq ! (from_str ("NaN") , Ok (Value :: Number (Number :: F32 (f32 :: NAN . into ())))) ; assert_eq ! (from_str ("+NaN") , Ok (Value :: Number (Number :: F32 (f32 :: NAN . into ())))) ; assert_eq ! (from_str ("-NaN") , Ok (Value :: Number (Number :: F32 ((- f32 :: NAN) . into ())))) ; assert_eq ! (from_str ("inf") , Ok (Value :: Number (Number :: F32 (f32 :: INFINITY . into ())))) ; assert_eq ! (from_str ("+inf") , Ok (Value :: Number (Number :: F32 (f32 :: INFINITY . into ())))) ; assert_eq ! (from_str ("-inf") , Ok (Value :: Number (Number :: F32 (f32 :: NEG_INFINITY . into ())))) ; }
};
}
