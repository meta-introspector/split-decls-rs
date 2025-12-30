// Generated macro for regions_that_outlive_free_regions (function)
macro_rules! Depcrate_type_check_livenessregions_that_outlive_free_regions {
() => {
// Module: crate::type_check::liveness
// Provides: {"regions_that_outlive_free_regions"}
// Dependencies: {}
# [doc = " Computes all regions that are (currently) known to outlive free"] # [doc = " regions. For these regions, we do not need to compute"] # [doc = " liveness, since the outlives constraints will ensure that they"] # [doc = " are live over the whole fn body anyhow."] fn regions_that_outlive_free_regions < 'tcx > (num_region_vars : usize , universal_regions : & UniversalRegions < 'tcx > , constraint_set : & OutlivesConstraintSet < 'tcx > ,) -> FxHashSet < RegionVid > { let rev_constraint_graph = constraint_set . reverse_graph (num_region_vars) ; let fr_static = universal_regions . fr_static ; let rev_region_graph = rev_constraint_graph . region_graph (constraint_set , fr_static) ; let mut stack : Vec < _ > = universal_regions . universal_regions_iter () . collect () ; let mut outlives_free_region : FxHashSet < _ > = stack . iter () . cloned () . collect () ; while let Some (sub_region) = stack . pop () { stack . extend (rev_region_graph . outgoing_regions (sub_region) . filter (| & r | outlives_free_region . insert (r)) ,) ; } outlives_free_region }
};
}
