// Generated macro for impl_23 (impl)
macro_rules! Depcrate_cursorimpl_23 {
() => {
// Module: crate::cursor
// Provides: {"impl_23"}
// Dependencies: {}
impl Drop for SyntaxNode { # [inline] fn drop (& mut self) { if self . data () . dec_rc () { unsafe { free (self . ptr) } } } }
};
}
