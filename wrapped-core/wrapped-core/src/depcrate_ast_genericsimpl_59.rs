// Generated macro for impl_59 (impl)
macro_rules! Depcrate_ast_genericsimpl_59 {
() => {
// Module: crate::ast::generics
// Provides: {"impl_59"}
// Dependencies: {}
impl < T , L , C > GenericParamExt for GenericParam < T , L , C > { type TypeParam = T ; type LifetimeParam = L ; type ConstParam = C ; fn as_type_param (& self) -> Option < & T > { if let GenericParam :: Type (ref val) = * self { Some (val) } else { None } } fn as_lifetime_param (& self) -> Option < & L > { if let GenericParam :: Lifetime (ref val) = * self { Some (val) } else { None } } fn as_const_param (& self) -> Option < & C > { if let GenericParam :: Const (ref val) = * self { Some (val) } else { None } } }
};
}
