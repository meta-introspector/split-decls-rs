// Generated macro for float_to_signed_int (function)
macro_rules! Depcrate_float_convfloat_to_signed_int {
() => {
// Module: crate::float::conv
// Provides: {"float_to_signed_int"}
// Dependencies: {}
# [doc = " Generic float to signed int conversions."] fn float_to_signed_int < F , I > (f : F) -> I where F : Float , I : Int + Neg < Output = I > , I :: Unsigned : Int , F :: Int : CastInto < I :: Unsigned > , F :: Int : CastFrom < u32 > , u32 : CastFrom < F :: Int > , { float_to_int_inner :: < F , I , _ , _ > (f . to_bits () & ! F :: SIGN_MASK , | i : I | if f . is_sign_negative () { - i } else { i } , | | if f . is_sign_negative () { I :: MIN } else { I :: MAX } ,) }
};
}
