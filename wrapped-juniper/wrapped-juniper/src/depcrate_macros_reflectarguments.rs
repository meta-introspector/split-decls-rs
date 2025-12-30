// Generated macro for Arguments (type)
macro_rules! Depcrate_macros_reflectArguments {
() => {
// Module: crate::macros::reflect
// Provides: {"Arguments"}
// Dependencies: {}
# [doc = " Alias for a slice of [field argument][1]s [`Name`], [`Type`] and"] # [doc = " [`WrappedValue`]."] # [doc = ""] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Language.Arguments"] pub type Arguments = & 'static [(Name , Type , WrappedValue)] ;
};
}
