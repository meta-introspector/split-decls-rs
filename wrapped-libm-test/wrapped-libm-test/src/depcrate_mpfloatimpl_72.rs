// Generated macro for impl_72 (impl)
macro_rules! Depcrate_mpfloatimpl_72 {
() => {
// Module: crate::mpfloat
// Provides: {"impl_72"}
// Dependencies: {}
impl MpOp for crate :: op :: lgamma_r :: Routine { type MpTy = MpFloat ; fn new_mp () -> Self :: MpTy { new_mpfloat :: < Self :: FTy > () } fn run (this : & mut Self :: MpTy , input : Self :: RustArgs) -> Self :: RustRet { this . assign (input . 0) ; let (sign , ord) = this . ln_abs_gamma_round (Nearest) ; let ret = prep_retval :: < Self :: FTy > (this , ord) ; (ret , sign as i32) } }
};
}
