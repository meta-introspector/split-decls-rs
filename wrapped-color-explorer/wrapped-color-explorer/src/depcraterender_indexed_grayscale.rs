// Generated macro for render_indexed_grayscale (function)
macro_rules! Depcraterender_indexed_grayscale {
() => {
// Module: crate
// Provides: {"render_indexed_grayscale"}
// Dependencies: {}
fn render_indexed_grayscale (frame : & mut Frame , area : Rect) { let layout = Layout :: vertical ([Constraint :: Length (1) , Constraint :: Length (1) ,]) . split (area) . iter () . flat_map (| area | { Layout :: horizontal ([Constraint :: Length (6) ; 12]) . split (* area) . to_vec () }) . collect_vec () ; for i in 232 ..= 255 { let color = Color :: Indexed (i) ; let color_index = format ! ("{i:0>3}") ; let bg = if i < 244 { Color :: Gray } else { Color :: Black } ; let paragraph = Paragraph :: new (Line :: from (vec ! [color_index . fg (color) . bg (bg) , "██" . bg (color) . fg (color) , "███████" . reversed () ,])) ; frame . render_widget (paragraph , layout [i as usize - 232]) ; } }
};
}
