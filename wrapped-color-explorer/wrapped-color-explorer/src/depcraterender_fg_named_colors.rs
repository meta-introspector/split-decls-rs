// Generated macro for render_fg_named_colors (function)
macro_rules! Depcraterender_fg_named_colors {
() => {
// Module: crate
// Provides: {"render_fg_named_colors"}
// Dependencies: {}
fn render_fg_named_colors (frame : & mut Frame , bg : Color , area : Rect) { let block = title_block (format ! ("Foreground colors on {bg} background")) ; let inner = block . inner (area) ; frame . render_widget (block , area) ; let vertical = Layout :: vertical ([Constraint :: Length (1) ; 2]) ; let horizontal = Layout :: horizontal ([Constraint :: Ratio (1 , 8) ; 8]) ; let areas = inner . layout_vec (& vertical) . into_iter () . flat_map (| area | area . layout_vec (& horizontal)) ; for (fg , area) in NAMED_COLORS . into_iter () . zip (areas) { let color_name = fg . to_string () ; let paragraph = Paragraph :: new (color_name) . fg (fg) . bg (bg) ; frame . render_widget (paragraph , area) ; } }
};
}
