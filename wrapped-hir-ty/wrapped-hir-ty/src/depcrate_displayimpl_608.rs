// Generated macro for impl_608 (impl)
macro_rules! Depcrate_displayimpl_608 {
() => {
// Module: crate::display
// Provides: {"impl_608"}
// Dependencies: {}
impl HirDisplay for OpaqueTy { fn hir_fmt (& self , f : & mut HirFormatter < '_ >) -> Result < () , HirDisplayError > { if f . should_truncate () { return write ! (f , "{TYPE_HINT_TRUNCATION}") ; } self . substitution . at (Interner , 0) . hir_fmt (f) } }
};
}
