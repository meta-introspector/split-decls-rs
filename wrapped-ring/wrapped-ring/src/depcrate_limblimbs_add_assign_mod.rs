// Generated macro for limbs_add_assign_mod (function)
macro_rules! Depcrate_limblimbs_add_assign_mod {
() => {
// Module: crate::limb
// Provides: {"limbs_add_assign_mod"}
// Dependencies: {}
# [inline] pub (crate) fn limbs_add_assign_mod (a : & mut [Limb] , b : & [Limb] , m : & [Limb] ,) -> Result < () , LenMismatchError > { prefixed_extern ! { fn LIMBS_add_mod (r : * mut Limb , a : * const Limb , b : * const Limb , m : * const Limb , num_limbs : c :: NonZero_size_t ,) ; } let num_limbs = NonZeroUsize :: new (m . len ()) . ok_or_else (| | LenMismatchError :: new (m . len ())) ? ; (InOut (a) , b) . with_non_dangling_non_null_pointers (num_limbs , | mut r , [a , b] | { let m = m . as_ptr () ; unsafe { LIMBS_add_mod (r . start_mut_ptr () , a , b , m , num_limbs) ; } }) }
};
}
