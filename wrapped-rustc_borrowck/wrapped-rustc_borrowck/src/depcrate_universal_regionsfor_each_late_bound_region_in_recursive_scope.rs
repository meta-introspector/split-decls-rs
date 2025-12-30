// Generated macro for for_each_late_bound_region_in_recursive_scope (function)
macro_rules! Depcrate_universal_regionsfor_each_late_bound_region_in_recursive_scope {
() => {
// Module: crate::universal_regions
// Provides: {"for_each_late_bound_region_in_recursive_scope"}
// Dependencies: {}
# [doc = " Iterates over the late-bound regions defined on `mir_def_id` and all of its"] # [doc = " parents, up to the typeck root, and invokes `f` with the liberated form"] # [doc = " of each one."] fn for_each_late_bound_region_in_recursive_scope < 'tcx > (tcx : TyCtxt < 'tcx > , mut mir_def_id : LocalDefId , mut f : impl FnMut (ty :: Region < 'tcx >) ,) { let typeck_root_def_id = tcx . typeck_root_def_id (mir_def_id . to_def_id ()) ; loop { for_each_late_bound_region_in_item (tcx , mir_def_id , & mut f) ; if mir_def_id . to_def_id () == typeck_root_def_id { break ; } else { mir_def_id = tcx . local_parent (mir_def_id) ; } } }
};
}
