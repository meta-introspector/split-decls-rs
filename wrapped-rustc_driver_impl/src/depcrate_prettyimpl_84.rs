// Generated macro for impl_84 (impl)
macro_rules! Depcrate_prettyimpl_84 {
() => {
// Module: crate::pretty
// Provides: {"impl_84"}
// Dependencies: {}
impl < 'tcx > PrintExtra < 'tcx > { fn with_krate < F , R > (& self , f : F) -> R where F : FnOnce (& ast :: Crate) -> R , { match self { PrintExtra :: AfterParsing { krate , .. } => f (krate) , PrintExtra :: NeedsAstMap { tcx } => f (& tcx . resolver_for_lowering () . borrow () . 1) , } } fn tcx (& self) -> TyCtxt < 'tcx > { match self { PrintExtra :: AfterParsing { .. } => bug ! ("PrintExtra::tcx") , PrintExtra :: NeedsAstMap { tcx } => * tcx , } } }
};
}
