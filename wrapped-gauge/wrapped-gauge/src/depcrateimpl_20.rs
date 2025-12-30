// Generated macro for impl_20 (impl)
macro_rules! Depcrateimpl_20 {
() => {
// Module: crate
// Provides: {"impl_20"}
// Dependencies: {}
impl Widget for & App { # [expect (clippy :: similar_names)] fn render (self , area : Rect , buf : & mut Buffer) { use Constraint :: { Length , Min , Ratio } ; let layout = Layout :: vertical ([Length (2) , Min (0) , Length (1)]) ; let [header_area , gauge_area , footer_area] = area . layout (& layout) ; let layout = Layout :: vertical ([Ratio (1 , 4) ; 4]) ; let [gauge1_area , gauge2_area , gauge3_area , gauge4_area] = gauge_area . layout (& layout) ; render_header (header_area , buf) ; render_footer (footer_area , buf) ; self . render_gauge1 (gauge1_area , buf) ; self . render_gauge2 (gauge2_area , buf) ; self . render_gauge3 (gauge3_area , buf) ; self . render_gauge4 (gauge4_area , buf) ; } }
};
}
