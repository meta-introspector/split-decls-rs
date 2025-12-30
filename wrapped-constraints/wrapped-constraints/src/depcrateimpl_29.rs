// Generated macro for impl_29 (impl)
macro_rules! Depcrateimpl_29 {
() => {
// Module: crate
// Provides: {"impl_29"}
// Dependencies: {}
impl Widget for SelectedTab { fn render (self , area : Rect , buf : & mut Buffer) { match self { Self :: Length => Self :: render_length_example (area , buf) , Self :: Percentage => Self :: render_percentage_example (area , buf) , Self :: Ratio => Self :: render_ratio_example (area , buf) , Self :: Fill => Self :: render_fill_example (area , buf) , Self :: Min => Self :: render_min_example (area , buf) , Self :: Max => Self :: render_max_example (area , buf) , } } }
};
}
