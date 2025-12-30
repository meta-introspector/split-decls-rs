// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
# [doc = " This implements `Widget` on a reference to the type, which means that it can be reused and"] # [doc = " doesn't need to be consumed when it is rendered. This is useful for widgets that need to store"] # [doc = " state and be updated over time."] # [doc = ""] # [doc = " This approach was probably always available in Ratatui, but it wasn't widely used until `Widget`"] # [doc = " was implemented on references in [PR #903] (merged in Ratatui 0.26.0). This is because all the"] # [doc = " built-in widgets previously would consume themselves when rendered."] impl Widget for & Timer { fn render (self , area : Rect , buf : & mut Buffer) { let elapsed = self . start . elapsed () . as_secs_f32 () ; let message = format ! ("Elapsed: {elapsed:.1?}s") ; message . render (area , buf) ; } }
};
}
