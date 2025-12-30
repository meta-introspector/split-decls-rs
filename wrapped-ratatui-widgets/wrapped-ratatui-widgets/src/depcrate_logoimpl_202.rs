// Generated macro for impl_202 (impl)
macro_rules! Depcrate_logoimpl_202 {
() => {
// Module: crate::logo
// Provides: {"impl_202"}
// Dependencies: {}
impl Widget for RatatuiLogo { fn render (self , area : Rect , buf : & mut Buffer) { let logo = self . size . as_str () ; Text :: raw (logo) . render (area , buf) ; } }
};
}
