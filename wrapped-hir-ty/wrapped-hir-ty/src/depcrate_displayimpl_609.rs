// Generated macro for impl_609 (impl)
macro_rules! Depcrate_displayimpl_609 {
() => {
// Module: crate::display
// Provides: {"impl_609"}
// Dependencies: {}
impl HirDisplay for GenericArg { fn hir_fmt (& self , f : & mut HirFormatter < '_ >) -> Result < () , HirDisplayError > { match self . interned () { crate :: GenericArgData :: Ty (ty) => ty . hir_fmt (f) , crate :: GenericArgData :: Lifetime (lt) => lt . hir_fmt (f) , crate :: GenericArgData :: Const (c) => c . hir_fmt (f) , } } }
};
}
