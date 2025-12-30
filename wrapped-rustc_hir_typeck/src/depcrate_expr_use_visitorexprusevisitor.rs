// Generated macro for ExprUseVisitor (struct)
macro_rules! Depcrate_expr_use_visitorExprUseVisitor {
() => {
// Module: crate::expr_use_visitor
// Provides: {"ExprUseVisitor"}
// Dependencies: {}
# [doc = " A visitor that reports how each expression is being used."] # [doc = ""] # [doc = " See [module-level docs][self] and [`Delegate`] for details."] pub struct ExprUseVisitor < 'tcx , Cx : TypeInformationCtxt < 'tcx > , D : Delegate < 'tcx > > { cx : Cx , # [doc = " We use a `RefCell` here so that delegates can mutate themselves, but we can"] # [doc = " still have calls to our own helper functions."] delegate : RefCell < D > , upvars : Option < & 'tcx FxIndexMap < HirId , hir :: Upvar > > , }
};
}
