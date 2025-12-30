// Generated macro for impl_58 (impl)
macro_rules! Depcrate_astimpl_58 {
() => {
// Module: crate::ast
// Provides: {"impl_58"}
// Dependencies: {}
impl GenericParam { pub fn span (& self) -> Span { match & self . kind { GenericParamKind :: Lifetime | GenericParamKind :: Type { default : None } => { self . ident . span } GenericParamKind :: Type { default : Some (ty) } => self . ident . span . to (ty . span) , GenericParamKind :: Const { span , .. } => * span , } } }
};
}
