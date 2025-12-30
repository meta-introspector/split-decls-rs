// Generated macro for compute_info_bound (function)
macro_rules! Depcrate_render_tui_draw_allcompute_info_bound {
() => {
// Module: crate::render::tui::draw::all
// Provides: {"compute_info_bound"}
// Dependencies: {}
fn compute_info_bound (bound : Rect , info : & [Line] , maximize : bool) -> (Rect , Option < Rect >) { if info . is_empty () { return (bound , None) ; } let margin = 1 ; let max_line_width = info . iter () . fold (0 , | state , l | { state . max (block_width (match l { Line :: Text (s) | Line :: Title (s) => s , }) + margin * 2 ,) }) ; let pane_width = if maximize { bound . width . saturating_sub (8) . min (max_line_width) } else { (bound . width / 3) . min (max_line_width) } ; if pane_width < max_line_width / 3 { return (bound , None) ; } (Rect { width : bound . width . saturating_sub (pane_width) , .. bound } , Some (rect :: snap_to_right (bound , pane_width)) ,) }
};
}
