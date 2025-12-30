// Generated macro for round_to_float (function)
macro_rules! Depcrate_lexical_roundinground_to_float {
() => {
// Module: crate::lexical::rounding
// Provides: {"round_to_float"}
// Dependencies: {}
# [inline] pub (crate) fn round_to_float < F , Algorithm > (fp : & mut ExtendedFloat , algorithm : Algorithm) where F : Float , Algorithm : FnOnce (& mut ExtendedFloat , i32) , { let final_exp = fp . exp + F :: DEFAULT_SHIFT ; if final_exp < F :: DENORMAL_EXPONENT { let diff = F :: DENORMAL_EXPONENT - fp . exp ; if diff <= u64 :: FULL { algorithm (fp , diff) ; } else { fp . mant = 0 ; fp . exp = 0 ; } } else { algorithm (fp , F :: DEFAULT_SHIFT) ; } if fp . mant & F :: CARRY_MASK == F :: CARRY_MASK { shr (fp , 1) ; } }
};
}
