// Generated macro for render_indexed_colors (function)
macro_rules! Depcraterender_indexed_colors {
() => {
// Module: crate
// Provides: {"render_indexed_colors"}
// Dependencies: {}
fn render_indexed_colors (frame : & mut Frame , area : Rect) { let block = title_block ("Indexed colors" . into ()) ; let inner = block . inner (area) ; frame . render_widget (block , area) ; let layout = Layout :: vertical ([Constraint :: Length (1) , Constraint :: Length (1) , Constraint :: Min (6) , Constraint :: Length (1) , Constraint :: Min (6) , Constraint :: Length (1) ,]) . split (inner) ; let color_layout = Layout :: horizontal ([Constraint :: Length (5) ; 16]) . split (layout [0]) ; for i in 0 .. 16 { let color = Color :: Indexed (i) ; let color_index = format ! ("{i:0>2}") ; let bg = if i < 1 { Color :: DarkGray } else { Color :: Black } ; let paragraph = Paragraph :: new (Line :: from (vec ! [color_index . fg (color) . bg (bg) , "██" . bg (color) . fg (color) ,])) ; frame . render_widget (paragraph , color_layout [i as usize]) ; } let index_layout = [layout [2] , layout [4]] . iter () . flat_map (| area | { Layout :: horizontal ([Constraint :: Length (27) ; 3]) . split (* area) . to_vec () }) . flat_map (| area | { Layout :: vertical ([Constraint :: Length (1) ; 6]) . split (area) . to_vec () }) . flat_map (| area | { Layout :: horizontal ([Constraint :: Min (4) ; 6]) . split (area) . to_vec () }) . collect_vec () ; for i in 16 ..= 231 { let color = Color :: Indexed (i) ; let color_index = format ! ("{i:0>3}") ; let paragraph = Paragraph :: new (Line :: from (vec ! [color_index . fg (color) . bg (Color :: Reset) , "." . bg (color) . fg (color) , "███" . reversed () ,])) ; frame . render_widget (paragraph , index_layout [i as usize - 16]) ; } }
};
}
