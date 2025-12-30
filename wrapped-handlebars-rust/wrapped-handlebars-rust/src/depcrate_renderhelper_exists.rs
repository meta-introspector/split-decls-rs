// Generated macro for helper_exists (function)
macro_rules! Depcrate_renderhelper_exists {
() => {
// Module: crate::render
// Provides: {"helper_exists"}
// Dependencies: {}
fn helper_exists < 'reg : 'rc , 'rc > (name : & str , reg : & Registry < 'reg > , rc : & RenderContext < 'reg , 'rc > ,) -> bool { rc . has_local_helper (name) || reg . has_helper (name) }
};
}
