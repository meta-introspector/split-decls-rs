// Generated macro for determine_cgu_reuse (function)
macro_rules! Depcrate_basedetermine_cgu_reuse {
() => {
// Module: crate::base
// Provides: {"determine_cgu_reuse"}
// Dependencies: {}
pub fn determine_cgu_reuse < 'tcx > (tcx : TyCtxt < 'tcx > , cgu : & CodegenUnit < 'tcx >) -> CguReuse { if ! tcx . dep_graph . is_fully_enabled () { return CguReuse :: No ; } let work_product_id = & cgu . work_product_id () ; if tcx . dep_graph . previous_work_product (work_product_id) . is_none () { return CguReuse :: No ; } let dep_node = cgu . codegen_dep_node (tcx) ; tcx . dep_graph . assert_dep_node_not_yet_allocated_in_current_session (& dep_node , | | { format ! ("CompileCodegenUnit dep-node for CGU `{}` already exists before marking." , cgu . name ()) }) ; if tcx . try_mark_green (& dep_node) { match compute_per_cgu_lto_type (& tcx . sess . lto () , & tcx . sess . opts , tcx . crate_types () , ModuleKind :: Regular ,) { ComputedLtoType :: No => CguReuse :: PostLto , _ => CguReuse :: PreLto , } } else { CguReuse :: No } }
};
}
