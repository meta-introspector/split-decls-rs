// Generated macro for ResolvedPattern (struct)
macro_rules! Depcrate_resolvingResolvedPattern {
() => {
// Module: crate::resolving
// Provides: {"ResolvedPattern"}
// Dependencies: {}
pub (crate) struct ResolvedPattern < 'db > { pub (crate) placeholders_by_stand_in : FxHashMap < SmolStr , parsing :: Placeholder > , pub (crate) node : SyntaxNode , pub (crate) resolved_paths : FxHashMap < SyntaxNode , ResolvedPath > , pub (crate) ufcs_function_calls : FxHashMap < SyntaxNode , UfcsCallInfo < 'db > > , pub (crate) contains_self : bool , }
};
}
