// Generated macro for GenericParamExt (trait)
macro_rules! Depcrate_ast_genericsGenericParamExt {
() => {
// Module: crate::ast::generics
// Provides: {"GenericParamExt"}
// Dependencies: {}
# [doc = " Extension trait for `GenericParam` to support getting values by variant."] # [doc = ""] # [doc = " # Usage"] # [doc = " `darling::ast::Generics` needs a way to test its params array in order to iterate over type params."] # [doc = " Rather than require callers to use `darling::ast::GenericParam` in all cases, this trait makes that"] # [doc = " polymorphic."] pub trait GenericParamExt { # [doc = " The type this GenericParam uses to represent type params and their bounds"] type TypeParam ; type LifetimeParam ; type ConstParam ; # [doc = " If this GenericParam is a type param, get the underlying value."] fn as_type_param (& self) -> Option < & Self :: TypeParam > { None } # [doc = " If this GenericParam is a lifetime, get the underlying value."] fn as_lifetime_param (& self) -> Option < & Self :: LifetimeParam > { None } # [doc = " If this GenericParam is a const param, get the underlying value."] fn as_const_param (& self) -> Option < & Self :: ConstParam > { None } }
};
}
