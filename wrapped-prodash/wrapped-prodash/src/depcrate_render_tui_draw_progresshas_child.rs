// Generated macro for has_child (function)
macro_rules! Depcrate_render_tui_draw_progresshas_child {
() => {
// Module: crate::render::tui::draw::progress
// Provides: {"has_child"}
// Dependencies: {}
fn has_child (entries : & [(Key , Task)] , index : usize) -> bool { entries . get (index + 1) . and_then (| (other_key , other_val) | { entries . get (index) . map (| (cur_key , _) | { cur_key . shares_parent_with (other_key , cur_key . level ()) && other_val . progress . is_some () }) }) . unwrap_or (false) }
};
}
