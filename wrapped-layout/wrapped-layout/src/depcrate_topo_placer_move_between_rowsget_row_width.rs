// Generated macro for get_row_width (function)
macro_rules! Depcrate_topo_placer_move_between_rowsget_row_width {
() => {
// Module: crate::topo::placer::move_between_rows
// Provides: {"get_row_width"}
// Dependencies: {}
# [doc = " Returns the sum of the width of the blocks in a row."] fn get_row_width (vg : & mut VisualGraph , idx : usize) -> f64 { let mut sum = 0. ; let row = vg . dag . row (idx) ; for elem in row { sum += vg . pos (* elem) . size (true) . x ; } sum }
};
}
