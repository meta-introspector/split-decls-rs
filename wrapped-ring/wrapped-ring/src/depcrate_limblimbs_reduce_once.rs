// Generated macro for limbs_reduce_once (function)
macro_rules! Depcrate_limblimbs_reduce_once {
() => {
// Module: crate::limb
// Provides: {"limbs_reduce_once"}
// Dependencies: {}
# [doc = " Equivalent to `if (r >= m) { r -= m; }`"] # [inline] pub fn limbs_reduce_once (r : & mut [Limb] , m : & [Limb]) -> Result < () , LenMismatchError > { prefixed_extern ! { fn LIMBS_reduce_once (r : * mut Limb , m : * const Limb , num_limbs : c :: NonZero_size_t) ; } let num_limbs = NonZeroUsize :: new (r . len ()) . ok_or_else (| | LenMismatchError :: new (m . len ())) ? ; let r = r . as_mut_ptr () ; let m = m . as_ptr () ; unsafe { LIMBS_reduce_once (r , m , num_limbs) } ; Ok (()) }
};
}
