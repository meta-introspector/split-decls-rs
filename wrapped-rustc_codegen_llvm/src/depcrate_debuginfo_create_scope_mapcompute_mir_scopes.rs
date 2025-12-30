// Generated macro for compute_mir_scopes (function)
macro_rules! Depcrate_debuginfo_create_scope_mapcompute_mir_scopes {
() => {
// Module: crate::debuginfo::create_scope_map
// Provides: {"compute_mir_scopes"}
// Dependencies: {}
# [doc = " Produces DIScope DIEs for each MIR Scope which has variables defined in it."] pub (crate) fn compute_mir_scopes < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , instance : Instance < 'tcx > , mir : & Body < 'tcx > , debug_context : & mut FunctionDebugContext < 'tcx , & 'll DIScope , & 'll DILocation > ,) { let variables = if cx . sess () . opts . debuginfo == DebugInfo :: Full { let mut vars = DenseBitSet :: new_empty (mir . source_scopes . len ()) ; for var_debug_info in & mir . var_debug_info { vars . insert (var_debug_info . source_info . scope) ; } Some (vars) } else { None } ; let mut instantiated = DenseBitSet :: new_empty (mir . source_scopes . len ()) ; let mut discriminators = FxHashMap :: default () ; for scope in mir . source_scopes . indices () { make_mir_scope (cx , instance , mir , & variables , debug_context , & mut instantiated , & mut discriminators , scope ,) ; } assert ! (instantiated . count () == mir . source_scopes . len ()) ; }
};
}
