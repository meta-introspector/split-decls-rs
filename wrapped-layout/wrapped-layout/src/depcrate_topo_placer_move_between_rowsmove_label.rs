// Generated macro for move_label (function)
macro_rules! Depcrate_topo_placer_move_between_rowsmove_label {
() => {
// Module: crate::topo::placer::move_between_rows
// Provides: {"move_label"}
// Dependencies: {}
# [doc = " Move the node label from curr to pred. Pred must have an empty label."] # [doc = " Return True if it was possible to move the labels."] fn move_label (vg : & mut VisualGraph , curr : NodeHandle , pred : NodeHandle ,) -> bool { let curr_shape = vg . element (curr) . shape . clone () ; let pred_shape = vg . element (pred) . shape . clone () ; if let ShapeKind :: Connector (txt) = pred_shape { if txt . is_some () { return false ; } } else { return false ; } if let ShapeKind :: Connector (txt) = curr_shape { if txt . is_none () { return false ; } vg . element_mut (pred) . shape = ShapeKind :: Connector (txt) ; vg . element_mut (curr) . shape = ShapeKind :: Connector (None) ; vg . element_mut (pred) . resize () ; vg . element_mut (curr) . resize () ; return true ; } false }
};
}
