// Generated macro for impl_530 (impl)
macro_rules! Depcrate_displayimpl_530 {
() => {
// Module: crate::display
// Provides: {"impl_530"}
// Dependencies: {}
impl < 'db , T : HirDisplay < 'db > > HirDisplay < 'db > for & T { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { HirDisplay :: hir_fmt (* self , f) } }
};
}
