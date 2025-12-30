// Generated macro for impl_216 (impl)
macro_rules! Depcrate_hirimpl_216 {
() => {
// Module: crate::hir
// Provides: {"impl_216"}
// Dependencies: {}
impl < 'hir , Unambig > ConstArg < 'hir , Unambig > { pub fn anon_const_hir_id (& self) -> Option < HirId > { match self . kind { ConstArgKind :: Anon (ac) => Some (ac . hir_id) , _ => None , } } pub fn span (& self) -> Span { match self . kind { ConstArgKind :: Path (path) => path . span () , ConstArgKind :: Anon (anon) => anon . span , ConstArgKind :: Infer (span , _) => span , } } }
};
}
