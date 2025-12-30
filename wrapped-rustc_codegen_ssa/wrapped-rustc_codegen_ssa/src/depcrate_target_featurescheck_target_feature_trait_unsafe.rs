// Generated macro for check_target_feature_trait_unsafe (function)
macro_rules! Depcrate_target_featurescheck_target_feature_trait_unsafe {
() => {
// Module: crate::target_features
// Provides: {"check_target_feature_trait_unsafe"}
// Dependencies: {}
# [doc = " Checks the function annotated with `#[target_feature]` is not a safe"] # [doc = " trait method implementation, reporting an error if it is."] pub (crate) fn check_target_feature_trait_unsafe (tcx : TyCtxt < '_ > , id : LocalDefId , attr_span : Span) { if let DefKind :: AssocFn = tcx . def_kind (id) { let parent_id = tcx . local_parent (id) ; if let DefKind :: Trait | DefKind :: Impl { of_trait : true } = tcx . def_kind (parent_id) { tcx . dcx () . emit_err (errors :: TargetFeatureSafeTrait { span : attr_span , def : tcx . def_span (id) , }) ; } } }
};
}
