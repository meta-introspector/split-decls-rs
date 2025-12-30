// Generated macro for provide (function)
macro_rules! Depcrate_baseprovide {
() => {
// Module: crate::base
// Provides: {"provide"}
// Dependencies: {}
pub (crate) fn provide (providers : & mut Providers) { providers . backend_optimization_level = | tcx , cratenum | { let for_speed = match tcx . sess . opts . optimize { config :: OptLevel :: No => return config :: OptLevel :: No , config :: OptLevel :: Less => return config :: OptLevel :: Less , config :: OptLevel :: More => return config :: OptLevel :: More , config :: OptLevel :: Aggressive => return config :: OptLevel :: Aggressive , config :: OptLevel :: Size => config :: OptLevel :: More , config :: OptLevel :: SizeMin => config :: OptLevel :: More , } ; let defids = tcx . collect_and_partition_mono_items (cratenum) . all_mono_items ; let any_for_speed = defids . items () . any (| id | { let CodegenFnAttrs { optimize , .. } = tcx . codegen_fn_attrs (* id) ; matches ! (optimize , OptimizeAttr :: Speed) }) ; if any_for_speed { return for_speed ; } tcx . sess . opts . optimize } ; }
};
}
