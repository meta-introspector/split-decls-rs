// Generated macro for draw_first_tab (function)
macro_rules! Depcrate_uidraw_first_tab {
() => {
// Module: crate::ui
// Provides: {"draw_first_tab"}
// Dependencies: {}
fn draw_first_tab (frame : & mut Frame , app : & mut App , area : Rect) { let chunks = Layout :: vertical ([Constraint :: Length (9) , Constraint :: Min (8) , Constraint :: Length (7) ,]) . split (area) ; draw_gauges (frame , app , chunks [0]) ; draw_charts (frame , app , chunks [1]) ; draw_text (frame , chunks [2]) ; }
};
}
