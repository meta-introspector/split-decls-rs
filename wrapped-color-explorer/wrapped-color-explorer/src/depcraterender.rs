// Generated macro for render (function)
macro_rules! Depcraterender {
() => {
// Module: crate
// Provides: {"render"}
// Dependencies: {}
fn render (frame : & mut Frame) { let [named , indexed_colors , indexed_greys] = Layout :: vertical ([Constraint :: Length (30) , Constraint :: Length (17) , Constraint :: Length (2) ,]) . areas (frame . area ()) ; render_named_colors (frame , named) ; render_indexed_colors (frame , indexed_colors) ; render_indexed_grayscale (frame , indexed_greys) ; }
};
}
