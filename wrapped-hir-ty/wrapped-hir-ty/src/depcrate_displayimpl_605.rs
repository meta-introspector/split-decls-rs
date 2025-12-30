// Generated macro for impl_605 (impl)
macro_rules! Depcrate_displayimpl_605 {
() => {
// Module: crate::display
// Provides: {"impl_605"}
// Dependencies: {}
impl < T : HirDisplay > HirDisplay for & T { fn hir_fmt (& self , f : & mut HirFormatter < '_ >) -> Result < () , HirDisplayError > { HirDisplay :: hir_fmt (* self , f) } }
};
}
