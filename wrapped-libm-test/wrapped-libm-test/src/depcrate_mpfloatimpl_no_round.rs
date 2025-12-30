// Generated macro for impl_no_round (macro)
macro_rules! Depcrate_mpfloatimpl_no_round {
() => {
// Module: crate::mpfloat
// Provides: {"impl_no_round"}
// Dependencies: {}
# [doc = " Implement unary functions that don't have a `_round` version"] macro_rules ! impl_no_round { ($ ($ fn_name : ident => $ rug_name : ident ;) *) => { paste :: paste ! { $ (impl_no_round ! { @ inner_unary $ fn_name , $ rug_name }) * } } ; (@ inner_unary $ fn_name : ident , $ rug_name : ident) => { impl MpOp for crate :: op ::$ fn_name :: Routine { type MpTy = MpFloat ; fn new_mp () -> Self :: MpTy { new_mpfloat ::< Self :: FTy > () } fn run (this : & mut Self :: MpTy , input : Self :: RustArgs) -> Self :: RustRet { this . assign (input . 0) ; this .$ rug_name () ; prep_retval ::< Self :: RustRet > (this , Ordering :: Equal) } } } ; }
};
}
