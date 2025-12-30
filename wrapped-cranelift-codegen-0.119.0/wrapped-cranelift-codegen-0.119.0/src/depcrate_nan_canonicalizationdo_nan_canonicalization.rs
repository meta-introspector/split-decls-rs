// Generated macro for do_nan_canonicalization (function)
macro_rules! Depcrate_nan_canonicalizationdo_nan_canonicalization {
() => {
// Module: crate::nan_canonicalization
// Provides: {"do_nan_canonicalization"}
// Dependencies: {}
# [doc = " Perform the NaN canonicalization pass."] pub fn do_nan_canonicalization (func : & mut Function , has_vector_support : bool) { let _tt = timing :: canonicalize_nans () ; let mut pos = FuncCursor :: new (func) ; while let Some (_block) = pos . next_block () { while let Some (inst) = pos . next_inst () { if is_fp_arith (& mut pos , inst) { add_nan_canon_seq (& mut pos , inst , has_vector_support) ; } } } }
};
}
