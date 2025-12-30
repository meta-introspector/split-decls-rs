// Generated macro for float_to_unsigned_int (function)
macro_rules! Depcrate_float_convfloat_to_unsigned_int {
() => {
// Module: crate::float::conv
// Provides: {"float_to_unsigned_int"}
// Dependencies: {}
# [doc = " Generic float to unsigned int conversions."] fn float_to_unsigned_int < F , U > (f : F) -> U where F : Float , U : Int < Unsigned = U > , F :: Int : CastInto < U > , F :: Int : CastFrom < u32 > , F :: Int : CastInto < U :: Unsigned > , u32 : CastFrom < F :: Int > , { float_to_int_inner :: < F , U , _ , _ > (f . to_bits () , | i : U | i , | | U :: MAX) }
};
}
