// Generated macro for impl_54 (impl)
macro_rules! Depcrate_ast_genericsimpl_54 {
() => {
// Module: crate::ast::generics
// Provides: {"impl_54"}
// Dependencies: {}
impl GenericParamExt for syn :: GenericParam { type TypeParam = syn :: TypeParam ; type LifetimeParam = syn :: LifetimeParam ; type ConstParam = syn :: ConstParam ; fn as_type_param (& self) -> Option < & Self :: TypeParam > { if let syn :: GenericParam :: Type (ref val) = * self { Some (val) } else { None } } fn as_lifetime_param (& self) -> Option < & Self :: LifetimeParam > { if let syn :: GenericParam :: Lifetime (ref val) = * self { Some (val) } else { None } } fn as_const_param (& self) -> Option < & Self :: ConstParam > { if let syn :: GenericParam :: Const (ref val) = * self { Some (val) } else { None } } }
};
}
