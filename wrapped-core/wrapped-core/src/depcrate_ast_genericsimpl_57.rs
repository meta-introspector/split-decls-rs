// Generated macro for impl_57 (impl)
macro_rules! Depcrate_ast_genericsimpl_57 {
() => {
// Module: crate::ast::generics
// Provides: {"impl_57"}
// Dependencies: {}
impl < T : FromTypeParam > FromTypeParam for GenericParam < T > { fn from_type_param (type_param : & syn :: TypeParam) -> Result < Self > { Ok (GenericParam :: Type (FromTypeParam :: from_type_param (type_param ,) ?)) } }
};
}
