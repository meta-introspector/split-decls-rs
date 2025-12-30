// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
impl Widget for & App { fn render (self , area : Rect , buf : & mut Buffer) { let [header_area , instructions_area , swap_legend_area , _ , blocks_area ,] = area . layout (& Layout :: vertical ([Length (2) , Length (2) , Length (1) , Length (1) , Fill (1) ,])) ; App :: header () . render (header_area , buf) ; App :: instructions () . render (instructions_area , buf) ; App :: swap_legend () . render (swap_legend_area , buf) ; self . render_layout_blocks (blocks_area , buf) ; } }
};
}
