// Generated macro for compute_mir_scopes (function)
macro_rules! Depcrate_debuginfocompute_mir_scopes {
() => {
// Module: crate::debuginfo
// Provides: {"compute_mir_scopes"}
// Dependencies: {}
# [doc = " Generate the `debug_context` in an MIR Body."] # [doc = " # Source of Origin"] # [doc = " Copied from `create_scope_map.rs` of rustc_codegen_llvm"] fn compute_mir_scopes < 'gcc , 'tcx > (cx : & CodegenCx < 'gcc , 'tcx > , instance : Instance < 'tcx > , mir : & Body < 'tcx > , debug_context : & mut FunctionDebugContext < 'tcx , () , Location < 'gcc > > ,) { let variables = if cx . sess () . opts . debuginfo == DebugInfo :: Full { let mut vars = DenseBitSet :: new_empty (mir . source_scopes . len ()) ; for var_debug_info in & mir . var_debug_info { vars . insert (var_debug_info . source_info . scope) ; } Some (vars) } else { None } ; let mut instantiated = DenseBitSet :: new_empty (mir . source_scopes . len ()) ; for idx in 0 .. mir . source_scopes . len () { let scope = SourceScope :: new (idx) ; make_mir_scope (cx , instance , mir , & variables , debug_context , & mut instantiated , scope) ; } assert ! (instantiated . count () == mir . source_scopes . len ()) ; }
};
}
