// Generated macro for MutablyUsedVariablesCtxt (struct)
macro_rules! Depcrate_needless_pass_by_ref_mutMutablyUsedVariablesCtxt {
() => {
// Module: crate::needless_pass_by_ref_mut
// Provides: {"MutablyUsedVariablesCtxt"}
// Dependencies: {}
struct MutablyUsedVariablesCtxt < 'tcx > { mutably_used_vars : HirIdSet , prev_bind : Option < HirId > , # [doc = " In async functions, the inner AST is composed of multiple layers until we reach the code"] # [doc = " defined by the user. Because of that, some variables are marked as mutably borrowed even"] # [doc = " though they're not. This field lists the `HirId` that should not be considered as mutable"] # [doc = " use of a variable."] prev_move_to_closure : HirIdSet , aliases : HirIdMap < HirId > , async_closures : FxIndexSet < LocalDefId > , tcx : TyCtxt < 'tcx > , }
};
}
