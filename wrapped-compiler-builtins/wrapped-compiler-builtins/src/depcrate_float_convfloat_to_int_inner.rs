// Generated macro for float_to_int_inner (function)
macro_rules! Depcrate_float_convfloat_to_int_inner {
() => {
// Module: crate::float::conv
// Provides: {"float_to_int_inner"}
// Dependencies: {}
# [doc = " Float to int conversions, generic for both signed and unsigned."] # [doc = ""] # [doc = " Parameters:"] # [doc = " - `fbits`: `abg(f)` bitcasted to an integer."] # [doc = " - `map_inbounds`: apply this transformation to integers that are within range (add the sign back)."] # [doc = " - `out_of_bounds`: return value when out of range for `I`."] fn float_to_int_inner < F , I , FnFoo , FnOob > (fbits : F :: Int , map_inbounds : FnFoo , out_of_bounds : FnOob ,) -> I where F : Float , I : Int , FnFoo : FnOnce (I) -> I , FnOob : FnOnce () -> I , I :: Unsigned : Int , F :: Int : CastInto < I :: Unsigned > , F :: Int : CastFrom < u32 > , u32 : CastFrom < F :: Int > , { let int_max_exp = F :: EXP_BIAS + I :: MAX . ilog2 () + 1 ; let foobar = F :: EXP_BIAS + I :: Unsigned :: BITS - 1 ; if fbits < F :: ONE . to_bits () { I :: ZERO } else if fbits < F :: Int :: cast_from (int_max_exp) << F :: SIG_BITS { let m_base = if I :: Unsigned :: BITS >= F :: Int :: BITS { I :: Unsigned :: cast_from (fbits) << (I :: BITS - F :: SIG_BITS - 1) } else { I :: Unsigned :: cast_from_lossy (fbits >> (F :: SIG_BITS - I :: BITS + 1)) } ; let m : I :: Unsigned = (I :: Unsigned :: ONE << (I :: BITS - 1)) | m_base ; let s : u32 = (foobar) - u32 :: cast_from (fbits >> F :: SIG_BITS) ; let unsigned = m >> s ; map_inbounds (I :: from_unsigned (unsigned)) } else if fbits <= F :: EXP_MASK { out_of_bounds () } else { I :: ZERO } }
};
}
