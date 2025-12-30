// Generated macro for render (function)
macro_rules! Depcrate_render_tui_enginerender {
() => {
// Module: crate::render::tui::engine
// Provides: {"render"}
// Dependencies: {}
# [doc = " An easy-to-use version of `render_with_input(…)` that does not allow state manipulation via an event stream."] pub fn render (out : impl std :: io :: Write , progress : impl WeakRoot , config : Options ,) -> Result < impl std :: future :: Future < Output = () > , std :: io :: Error > { render_with_input (out , progress , config , futures_lite :: stream :: pending ()) }
};
}
