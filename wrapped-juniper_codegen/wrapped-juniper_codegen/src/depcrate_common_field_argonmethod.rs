// Generated macro for OnMethod (enum)
macro_rules! Depcrate_common_field_argOnMethod {
() => {
// Module: crate::common::field::arg
// Provides: {"OnMethod"}
// Dependencies: {}
# [doc = " Possible kinds of Rust method arguments for code generation."] # [derive (Debug)] pub (crate) enum OnMethod { # [doc = " Regular [GraphQL field argument][1]."] # [doc = ""] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Language.Arguments"] Regular (Box < OnField >) , # [doc = " [`Context`] passed into a [GraphQL field][2] resolving method."] # [doc = ""] # [doc = " [`Context`]: juniper::Context"] # [doc = " [2]: https://spec.graphql.org/October2021#sec-Language.Fields"] Context (Box < syn :: Type >) , # [doc = " [`Executor`] passed into a [GraphQL field][2] resolving method."] # [doc = ""] # [doc = " [`Executor`]: juniper::Executor"] # [doc = " [2]: https://spec.graphql.org/October2021#sec-Language.Fields"] Executor , }
};
}
