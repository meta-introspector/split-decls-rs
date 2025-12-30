// Generated macro for impl_58 (impl)
macro_rules! Depcrate_ast_genericsimpl_58 {
() => {
// Module: crate::ast::generics
// Provides: {"impl_58"}
// Dependencies: {}
impl < T : FromTypeParam > FromGenericParam for GenericParam < T > { fn from_generic_param (param : & syn :: GenericParam) -> Result < Self > { Ok (match * param { syn :: GenericParam :: Type (ref ty) => { GenericParam :: Type (FromTypeParam :: from_type_param (ty) ?) } syn :: GenericParam :: Lifetime (ref val) => GenericParam :: Lifetime (val . clone ()) , syn :: GenericParam :: Const (ref val) => GenericParam :: Const (val . clone ()) , }) } }
};
}
