// Generated macro for fixed_powi_value (function)
macro_rules! Depcrate_mathfixed_powi_value {
() => {
// Module: crate::math
// Provides: {"fixed_powi_value"}
// Dependencies: {}
# [doc = " Returns `Some(output)` if `powi` (called `pown` in C) results in a fixed value specified in the"] # [doc = " C standard (specifically, C23 annex F.10.4.6) when doing `base^exp`. Otherwise, returns `None`."] pub (crate) fn fixed_powi_value < S : Semantics > (ecx : & mut MiriInterpCx < '_ > , base : IeeeFloat < S > , exp : i32 ,) -> Option < IeeeFloat < S > > where IeeeFloat < S > : IeeeExt , { match exp { 0 => { let one = IeeeFloat :: < S > :: one () ; let rng = ecx . machine . rng . get_mut () ; let return_nan = ecx . machine . float_nondet && rng . random () && base . is_signaling () ; Some (if return_nan { ecx . generate_nan (& [base]) } else { one }) } _ => return None , } }
};
}
