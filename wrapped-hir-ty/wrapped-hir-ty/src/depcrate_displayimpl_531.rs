// Generated macro for impl_531 (impl)
macro_rules! Depcrate_displayimpl_531 {
() => {
// Module: crate::display
// Provides: {"impl_531"}
// Dependencies: {}
impl < 'db , T : HirDisplay < 'db > + Internable > HirDisplay < 'db > for Interned < T > { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { HirDisplay :: hir_fmt (self . as_ref () , f) } }
};
}
