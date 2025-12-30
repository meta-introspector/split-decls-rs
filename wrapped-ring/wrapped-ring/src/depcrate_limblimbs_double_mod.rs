// Generated macro for limbs_double_mod (function)
macro_rules! Depcrate_limblimbs_double_mod {
() => {
// Module: crate::limb
// Provides: {"limbs_double_mod"}
// Dependencies: {}
pub (crate) fn limbs_double_mod (r : & mut [Limb] , m : & [Limb]) -> Result < () , LenMismatchError > { prefixed_extern ! { fn LIMBS_shl_mod (r : * mut Limb , a : * const Limb , m : * const Limb , num_limbs : c :: NonZero_size_t) ; } let num_limbs = NonZeroUsize :: new (m . len ()) . ok_or_else (| | LenMismatchError :: new (m . len ())) ? ; r . with_non_dangling_non_null_pointers (num_limbs , | mut r , [a] | { let m = m . as_ptr () ; unsafe { LIMBS_shl_mod (r . start_mut_ptr () , a , m , num_limbs) ; } }) . map (| _ | ()) }
};
}
