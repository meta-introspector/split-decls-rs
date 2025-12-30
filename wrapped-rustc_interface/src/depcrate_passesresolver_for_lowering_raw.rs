// Generated macro for resolver_for_lowering_raw (function)
macro_rules! Depcrate_passesresolver_for_lowering_raw {
() => {
// Module: crate::passes
// Provides: {"resolver_for_lowering_raw"}
// Dependencies: {}
fn resolver_for_lowering_raw < 'tcx > (tcx : TyCtxt < 'tcx > , () : () ,) -> (& 'tcx Steal < (ty :: ResolverAstLowering , Arc < ast :: Crate >) > , & 'tcx ty :: ResolverGlobalCtxt) { let arenas = Resolver :: arenas () ; let _ = tcx . registered_tools (()) ; let (krate , pre_configured_attrs) = tcx . crate_for_resolver (()) . steal () ; let mut resolver = Resolver :: new (tcx , & pre_configured_attrs , krate . spans . inner_span , krate . spans . inject_use_span , & arenas ,) ; let krate = configure_and_expand (krate , & pre_configured_attrs , & mut resolver) ; tcx . untracked () . cstore . freeze () ; let ResolverOutputs { global_ctxt : untracked_resolutions , ast_lowering : untracked_resolver_for_lowering , } = resolver . into_outputs () ; let resolutions = tcx . arena . alloc (untracked_resolutions) ; (tcx . arena . alloc (Steal :: new ((untracked_resolver_for_lowering , Arc :: new (krate)))) , resolutions) }
};
}
