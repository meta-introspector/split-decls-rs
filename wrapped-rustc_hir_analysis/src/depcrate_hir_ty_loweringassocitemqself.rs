// Generated macro for AssocItemQSelf (enum)
macro_rules! Depcrate_hir_ty_loweringAssocItemQSelf {
() => {
// Module: crate::hir_ty_lowering
// Provides: {"AssocItemQSelf"}
// Dependencies: {}
# [doc = " The \"qualified self\" of an associated item path."] # [doc = ""] # [doc = " For diagnostic purposes only."] enum AssocItemQSelf { Trait (DefId) , TyParam (LocalDefId , Span) , SelfTyAlias , }
};
}
