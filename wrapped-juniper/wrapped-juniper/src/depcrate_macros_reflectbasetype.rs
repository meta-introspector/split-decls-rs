// Generated macro for BaseType (trait)
macro_rules! Depcrate_macros_reflectBaseType {
() => {
// Module: crate::macros::reflect
// Provides: {"BaseType"}
// Dependencies: {}
# [doc = " Naming of a [GraphQL object][1], [scalar][2] or [interface][3] [`Type`]."] # [doc = ""] # [doc = " This trait is transparent to [`Option`], [`Vec`] and other containers, so to"] # [doc = " fully represent a [GraphQL object][1] we additionally use [`WrappedType`]."] # [doc = ""] # [doc = " Different Rust types may have the same [`NAME`]. For example, [`String`] and"] # [doc = " `&`[`str`](prim@str) share `String!` GraphQL type."] # [doc = ""] # [doc = " [`NAME`]: Self::NAME"] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Objects"] # [doc = " [2]: https://spec.graphql.org/October2021#sec-Scalars"] # [doc = " [3]: https://spec.graphql.org/October2021#sec-Interfaces"] pub trait BaseType < S > { # [doc = " [`Type`] of the [GraphQL object][1], [scalar][2] or [interface][3]."] # [doc = ""] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Objects"] # [doc = " [2]: https://spec.graphql.org/October2021#sec-Scalars"] # [doc = " [3]: https://spec.graphql.org/October2021#sec-Interfaces"] const NAME : Type ; }
};
}
