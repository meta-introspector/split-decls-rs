// Generated macro for LateContext (struct)
macro_rules! Depcrate_contextLateContext {
() => {
// Module: crate::context
// Provides: {"LateContext"}
// Dependencies: {}
# [doc = " Context for lint checking outside of type inference."] pub struct LateContext < 'tcx > { # [doc = " Type context we're checking in."] pub tcx : TyCtxt < 'tcx > , # [doc = " Current body, or `None` if outside a body."] pub enclosing_body : Option < hir :: BodyId > , # [doc = " Type-checking results for the current body. Access using the `typeck_results`"] # [doc = " and `maybe_typeck_results` methods, which handle querying the typeck results on demand."] pub (super) cached_typeck_results : Cell < Option < & 'tcx ty :: TypeckResults < 'tcx > > > , # [doc = " Parameter environment for the item we are in."] pub param_env : ty :: ParamEnv < 'tcx > , # [doc = " Items accessible from the crate being checked."] pub effective_visibilities : & 'tcx EffectiveVisibilities , pub last_node_with_lint_attrs : hir :: HirId , # [doc = " Generic type parameters in scope for the item we are in."] pub generics : Option < & 'tcx hir :: Generics < 'tcx > > , # [doc = " We are only looking at one module"] pub only_module : bool , }
};
}
