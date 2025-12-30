// Generated macro for do_it (function)
macro_rules! Depcrate_topo_placer_move_between_rowsdo_it {
() => {
// Module: crate::topo::placer::move_between_rows
// Provides: {"do_it"}
// Dependencies: {}
# [cfg_attr (not (feature = "log") , allow (unused_assignments , unused_variables))] pub (crate) fn do_it (vg : & mut VisualGraph) { let mut cnt = 0 ; for _ in 0 .. 3 { cnt += move_text_up (vg) ; } # [cfg (feature = "log")] log :: info ! ("Moved {} labels between rows" , cnt) ; }
};
}
