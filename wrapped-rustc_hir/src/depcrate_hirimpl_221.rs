// Generated macro for impl_221 (impl)
macro_rules! Depcrate_hirimpl_221 {
() => {
// Module: crate::hir
// Provides: {"impl_221"}
// Dependencies: {}
impl GenericArg < '_ > { pub fn span (& self) -> Span { match self { GenericArg :: Lifetime (l) => l . ident . span , GenericArg :: Type (t) => t . span , GenericArg :: Const (c) => c . span () , GenericArg :: Infer (i) => i . span , } } pub fn hir_id (& self) -> HirId { match self { GenericArg :: Lifetime (l) => l . hir_id , GenericArg :: Type (t) => t . hir_id , GenericArg :: Const (c) => c . hir_id , GenericArg :: Infer (i) => i . hir_id , } } pub fn descr (& self) -> & 'static str { match self { GenericArg :: Lifetime (_) => "lifetime" , GenericArg :: Type (_) => "type" , GenericArg :: Const (_) => "constant" , GenericArg :: Infer (_) => "placeholder" , } } pub fn to_ord (& self) -> ast :: ParamKindOrd { match self { GenericArg :: Lifetime (_) => ast :: ParamKindOrd :: Lifetime , GenericArg :: Type (_) | GenericArg :: Const (_) | GenericArg :: Infer (_) => { ast :: ParamKindOrd :: TypeOrConst } } } pub fn is_ty_or_const (& self) -> bool { match self { GenericArg :: Lifetime (_) => false , GenericArg :: Type (_) | GenericArg :: Const (_) | GenericArg :: Infer (_) => true , } } }
};
}
