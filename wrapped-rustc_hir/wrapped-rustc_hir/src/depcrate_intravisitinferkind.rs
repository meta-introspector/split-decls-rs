// Generated macro for InferKind (enum)
macro_rules! Depcrate_intravisitInferKind {
() => {
// Module: crate::intravisit
// Provides: {"InferKind"}
// Dependencies: {}
# [doc = " We track whether an infer var is from a [`Ty`], [`ConstArg`], or [`GenericArg`] so that"] # [doc = " HIR visitors overriding [`Visitor::visit_infer`] can determine what kind of infer is being visited"] pub enum InferKind < 'hir > { Ty (& 'hir Ty < 'hir >) , Const (& 'hir ConstArg < 'hir >) , Ambig (& 'hir InferArg) , }
};
}
