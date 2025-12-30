// Generated macro for impl_570 (impl)
macro_rules! Depcrate_inferimpl_570 {
() => {
// Module: crate::infer
// Provides: {"impl_570"}
// Dependencies: {}
impl < 'tcx > TypeFolder < TyCtxt < 'tcx > > for InferenceLiteralEraser < 'tcx > { fn cx (& self) -> TyCtxt < 'tcx > { self . tcx } fn fold_ty (& mut self , ty : Ty < 'tcx >) -> Ty < 'tcx > { match ty . kind () { ty :: Infer (ty :: IntVar (_) | ty :: FreshIntTy (_)) => self . tcx . types . i32 , ty :: Infer (ty :: FloatVar (_) | ty :: FreshFloatTy (_)) => self . tcx . types . f64 , _ => ty . super_fold_with (self) , } } }
};
}
