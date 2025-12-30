// Generated macro for impl_130 (impl)
macro_rules! Depcrate_render_line_engineimpl_130 {
() => {
// Module: crate::render::line::engine
// Provides: {"impl_130"}
// Dependencies: {}
impl Drop for JoinHandle { fn drop (& mut self) { self . shutdown () ; self . inner . take () . and_then (| h | h . join () . ok ()) ; } }
};
}
