// Generated macro for limbs_less_than_limbs (function)
macro_rules! Depcrate_limblimbs_less_than_limbs {
() => {
// Module: crate::limb
// Provides: {"limbs_less_than_limbs"}
// Dependencies: {}
# [inline] fn limbs_less_than_limbs (a : & [Limb] , b : & [Limb]) -> Result < LimbMask , LenMismatchError > { prefixed_extern ! { fn LIMBS_less_than (a : * const Limb , b : * const Limb , num_limbs : c :: NonZero_size_t) -> LimbMask ; } let len = NonZeroUsize :: new (b . len ()) . ok_or_else (| | LenMismatchError :: new (a . len ())) ? ; if a . len () != len . get () { return Err (LenMismatchError :: new (a . len ())) ; } Ok (unsafe { LIMBS_less_than (a . as_ptr () , b . as_ptr () , len) }) }
};
}
