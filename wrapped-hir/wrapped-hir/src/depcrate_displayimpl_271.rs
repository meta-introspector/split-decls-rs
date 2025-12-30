// Generated macro for impl_271 (impl)
macro_rules! Depcrate_displayimpl_271 {
() => {
// Module: crate::display
// Provides: {"impl_271"}
// Dependencies: {}
impl < 'db > HirDisplay < 'db > for Adt { fn hir_fmt (& self , f : & mut HirFormatter < '_ , 'db >) -> Result < () , HirDisplayError > { match self { Adt :: Struct (it) => it . hir_fmt (f) , Adt :: Union (it) => it . hir_fmt (f) , Adt :: Enum (it) => it . hir_fmt (f) , } } }
};
}
