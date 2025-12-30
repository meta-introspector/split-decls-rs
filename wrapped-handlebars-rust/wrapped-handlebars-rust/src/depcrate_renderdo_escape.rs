// Generated macro for do_escape (function)
macro_rules! Depcrate_renderdo_escape {
() => {
// Module: crate::render
// Provides: {"do_escape"}
// Dependencies: {}
pub (crate) fn do_escape (r : & Registry < '_ > , rc : & RenderContext < '_ , '_ > , content : String) -> String { if ! rc . is_disable_escape () { r . get_escape_fn () (& content) } else { content } }
};
}
