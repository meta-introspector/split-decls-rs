// Generated macro for impl_310 (impl)
macro_rules! Depcrate_topo_placer_placeimpl_310 {
() => {
// Module: crate::topo::placer::place
// Provides: {"impl_310"}
// Dependencies: {}
impl < 'a > Placer < 'a > { pub fn new (vg : & 'a mut VisualGraph) -> Self { Self { vg } } pub fn layout (& mut self , no_layout : bool) { # [cfg (feature = "log")] log :: info ! ("Starting layout of {} nodes. " , self . vg . num_nodes ()) ; let need_transpose = ! self . vg . orientation () . is_top_to_bottom () ; if need_transpose { # [cfg (feature = "log")] log :: info ! ("Placing nodes in Left-to-right mode.") ; self . vg . transpose () ; } else { # [cfg (feature = "log")] log :: info ! ("Placing nodes in Top-to-Bottom mode.") ; } move_between_rows :: do_it (self . vg) ; simple :: do_it (self . vg) ; verifier :: do_it (self . vg) ; if no_layout { # [cfg (feature = "log")] log :: info ! ("Skipping the layout phase.") ; if need_transpose { self . vg . transpose () ; } return ; } BK :: new (self . vg) . do_it () ; verifier :: do_it (self . vg) ; edge_fixer :: do_it (self . vg) ; if need_transpose { self . vg . transpose () ; } } }
};
}
