// Generated macro for render_named_colors (function)
macro_rules! Depcraterender_named_colors {
() => {
// Module: crate
// Provides: {"render_named_colors"}
// Dependencies: {}
fn render_named_colors (frame : & mut Frame , area : Rect) { let layout = Layout :: vertical ([Constraint :: Length (3) ; 10]) . split (area) ; render_fg_named_colors (frame , Color :: Reset , layout [0]) ; render_fg_named_colors (frame , Color :: Black , layout [1]) ; render_fg_named_colors (frame , Color :: DarkGray , layout [2]) ; render_fg_named_colors (frame , Color :: Gray , layout [3]) ; render_fg_named_colors (frame , Color :: White , layout [4]) ; render_bg_named_colors (frame , Color :: Reset , layout [5]) ; render_bg_named_colors (frame , Color :: Black , layout [6]) ; render_bg_named_colors (frame , Color :: DarkGray , layout [7]) ; render_bg_named_colors (frame , Color :: Gray , layout [8]) ; render_bg_named_colors (frame , Color :: White , layout [9]) ; }
};
}
