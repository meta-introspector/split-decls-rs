// Generated macro for TyOrConstInferVar (enum)
macro_rules! Depcrate_inferTyOrConstInferVar {
() => {
// Module: crate::infer
// Provides: {"TyOrConstInferVar"}
// Dependencies: {}
# [doc = " Helper for [InferCtxt::ty_or_const_infer_var_changed] (see comment on that), currently"] # [doc = " used only for `traits::fulfill`'s list of `stalled_on` inference variables."] # [derive (Copy , Clone , Debug)] pub enum TyOrConstInferVar { # [doc = " Equivalent to `ty::Infer(ty::TyVar(_))`."] Ty (TyVid) , # [doc = " Equivalent to `ty::Infer(ty::IntVar(_))`."] TyInt (IntVid) , # [doc = " Equivalent to `ty::Infer(ty::FloatVar(_))`."] TyFloat (FloatVid) , # [doc = " Equivalent to `ty::ConstKind::Infer(ty::InferConst::Var(_))`."] Const (ConstVid) , }
};
}
