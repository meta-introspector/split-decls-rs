// Generated macro for GenericsExt (trait)
macro_rules! Depcrate_usage_generics_extGenericsExt {
() => {
// Module: crate::usage::generics_ext
// Provides: {"GenericsExt"}
// Dependencies: {}
# [doc = " Extension trait for pulling specific generics data from a generics AST representation."] pub trait GenericsExt { # [doc = " Get the set of all lifetimes declared by the syntax element."] # [doc = " This does not look for usage of the lifetime; see `UsesLifetimes` for that."] fn declared_lifetimes (& self) -> LifetimeSet ; # [doc = " Get the set of all type parameters declared by the syntax element."] # [doc = " This does not look for usage of the type parameter; see `UsesTypeParams` for that."] fn declared_type_params (& self) -> IdentSet ; }
};
}
