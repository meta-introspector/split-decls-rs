// Generated macro for str_width (function)
macro_rules! Depcrate_renderer_renderstr_width {
() => {
// Module: crate::renderer::render
// Provides: {"str_width"}
// Dependencies: {}
fn str_width (s : & str) -> usize { s . chars () . map (char_width) . sum () }
};
}
