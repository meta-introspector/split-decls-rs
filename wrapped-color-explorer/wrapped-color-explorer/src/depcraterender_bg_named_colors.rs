// Generated macro for render_bg_named_colors (function)
macro_rules! Depcraterender_bg_named_colors {
() => {
// Module: crate
// Provides: {"render_bg_named_colors"}
// Dependencies: {}
fn render_bg_named_colors (frame : & mut Frame , fg : Color , area : Rect) { let block = title_block (format ! ("Background colors with {fg} foreground")) ; let inner = block . inner (area) ; frame . render_widget (block , area) ; let vertical = Layout :: vertical ([Constraint :: Length (1) ; 2]) ; let horizontal = Layout :: horizontal ([Constraint :: Ratio (1 , 8) ; 8]) ; let areas = inner . layout_vec (& vertical) . into_iter () . flat_map (| area | area . layout_vec (& horizontal)) ; for (bg , area) in NAMED_COLORS . into_iter () . zip (areas) { let color_name = bg . to_string () ; let paragraph = Paragraph :: new (color_name) . fg (fg) . bg (bg) ; frame . render_widget (paragraph , area) ; } }
};
}
