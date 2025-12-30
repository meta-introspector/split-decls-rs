// Generated macro for impl_26 (impl)
macro_rules! Depcrate_cursorimpl_26 {
() => {
// Module: crate::cursor
// Provides: {"impl_26"}
// Dependencies: {}
impl Drop for SyntaxToken { # [inline] fn drop (& mut self) { if self . data () . dec_rc () { unsafe { free (self . ptr) } } } }
};
}
