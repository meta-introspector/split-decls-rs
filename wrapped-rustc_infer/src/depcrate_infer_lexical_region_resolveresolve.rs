// Generated macro for resolve (function)
macro_rules! Depcrate_infer_lexical_region_resolveresolve {
() => {
// Module: crate::infer::lexical_region_resolve
// Provides: {"resolve"}
// Dependencies: {}
# [doc = " This function performs lexical region resolution given a complete"] # [doc = " set of constraints and variable origins. It performs a fixed-point"] # [doc = " iteration to find region values which satisfy all constraints,"] # [doc = " assuming such values can be found. It returns the final values of"] # [doc = " all the variables as well as a set of errors that must be reported."] # [instrument (level = "debug" , skip (region_rels , var_infos , data))] pub (crate) fn resolve < 'tcx > (region_rels : & RegionRelations < '_ , 'tcx > , var_infos : VarInfos , data : RegionConstraintData < 'tcx > ,) -> (LexicalRegionResolutions < 'tcx > , Vec < RegionResolutionError < 'tcx > >) { let mut errors = vec ! [] ; let mut resolver = LexicalResolver { region_rels , var_infos , data } ; let values = resolver . infer_variable_values (& mut errors) ; (values , errors) }
};
}
