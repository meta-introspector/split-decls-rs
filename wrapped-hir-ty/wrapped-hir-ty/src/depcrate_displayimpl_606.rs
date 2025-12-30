// Generated macro for impl_606 (impl)
macro_rules! Depcrate_displayimpl_606 {
() => {
// Module: crate::display
// Provides: {"impl_606"}
// Dependencies: {}
impl < T : HirDisplay + Internable > HirDisplay for Interned < T > { fn hir_fmt (& self , f : & mut HirFormatter < '_ >) -> Result < () , HirDisplayError > { HirDisplay :: hir_fmt (self . as_ref () , f) } }
};
}
