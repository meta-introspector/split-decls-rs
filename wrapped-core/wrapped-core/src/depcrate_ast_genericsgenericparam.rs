// Generated macro for GenericParam (enum)
macro_rules! Depcrate_ast_genericsGenericParam {
() => {
// Module: crate::ast::generics
// Provides: {"GenericParam"}
// Dependencies: {}
# [doc = " A mirror of `syn::GenericParam` which is generic over all its contents."] # [derive (Debug , Clone , PartialEq , Eq)] pub enum GenericParam < T = syn :: TypeParam , L = syn :: LifetimeParam , C = syn :: ConstParam > { Type (T) , Lifetime (L) , Const (C) , }
};
}
