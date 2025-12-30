// Generated macro for ExprFnSig (enum)
macro_rules! Depcrate_tyExprFnSig {
() => {
// Module: crate::ty
// Provides: {"ExprFnSig"}
// Dependencies: {}
# [doc = " A signature for a function like type."] # [derive (Clone , Copy , Debug)] pub enum ExprFnSig < 'tcx > { Sig (Binder < 'tcx , FnSig < 'tcx > > , Option < DefId >) , Closure (Option < & 'tcx FnDecl < 'tcx > > , Binder < 'tcx , FnSig < 'tcx > >) , Trait (Binder < 'tcx , Ty < 'tcx > > , Option < Binder < 'tcx , Ty < 'tcx > > > , Option < DefId >) , }
};
}
