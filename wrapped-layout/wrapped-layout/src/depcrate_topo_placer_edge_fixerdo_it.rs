// Generated macro for do_it (function)
macro_rules! Depcrate_topo_placer_edge_fixerdo_it {
() => {
// Module: crate::topo::placer::edge_fixer
// Provides: {"do_it"}
// Dependencies: {}
# [cfg_attr (not (feature = "log") , allow (unused_assignments , unused_variables))] pub (crate) fn do_it (vg : & mut VisualGraph) { let mut cnt = 0 ; cnt += handle_disconnected_nodes (vg) ; cnt += align_self_edges (vg) ; align_to_left (vg) ; # [cfg (feature = "log")] log :: info ! ("Aligned {} edges." , cnt) ; cnt = straighten_edge (vg) ; # [cfg (feature = "log")] log :: info ! ("Straightened {} edges." , cnt) ; cnt = adjust_crossing_edges (vg) ; # [cfg (feature = "log")] log :: info ! ("Adjusted crossing {} edges." , cnt) ; }
};
}
