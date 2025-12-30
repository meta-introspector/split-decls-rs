// Generated macro for render (function)
macro_rules! Depcraterender {
() => {
// Module: crate
// Provides: {"render"}
// Dependencies: {}
fn render (frame : & mut Frame) { let area = frame . area () ; frame . buffer_mut () . set_style (area , (FG_COLOR , BG_COLOR)) ; let logo_width = 29 ; let menu_width = 23 ; let padding = 2 ; let menu_borders = 3 ; let height = MAIN_DISHES . len () as u16 + BACKENDS . len () as u16 + menu_borders ; let width = logo_width + menu_width + padding ; let center_area = area . centered (Constraint :: Length (width) , Constraint :: Length (height)) ; let layout = Layout :: horizontal (Constraint :: from_lengths ([logo_width , padding , menu_width])) ; let [logo_area , _ , menu_area] = center_area . layout (& layout) ; render_logo (frame , logo_area) ; render_menu (frame , menu_area) ; }
};
}
