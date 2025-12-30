// Generated macro for prep_retval (function)
macro_rules! Depcrate_mpfloatprep_retval {
() => {
// Module: crate::mpfloat
// Provides: {"prep_retval"}
// Dependencies: {}
# [doc = " Set subnormal emulation and convert to a concrete float type."] fn prep_retval < F : Float > (mp : & mut MpFloat , ord : Ordering) -> F where for < 'a > & 'a MpFloat : az :: Cast < F > , { mp . subnormalize_ieee_round (ord , Nearest) ; (& * mp) . az :: < F > () }
};
}
