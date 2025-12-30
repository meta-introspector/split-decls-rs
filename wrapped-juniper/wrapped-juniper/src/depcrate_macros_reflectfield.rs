// Generated macro for Field (trait)
macro_rules! Depcrate_macros_reflectField {
() => {
// Module: crate::macros::reflect
// Provides: {"Field"}
// Dependencies: {}
# [doc = " Synchronous field of a [GraphQL object][1] or [interface][2]."] # [doc = ""] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Objects"] # [doc = " [2]: https://spec.graphql.org/October2021#sec-Interfaces"] pub trait Field < S , const N : FieldName > : FieldMeta < S , N > { # [doc = " Resolves the [`Value`] of this synchronous [`Field`]."] # [doc = ""] # [doc = " The `arguments` object contains all the specified arguments, with the"] # [doc = " default values being substituted for the ones not provided by the query."] # [doc = ""] # [doc = " The `executor` can be used to drive selections into sub-[objects][1]."] # [doc = ""] # [doc = " [`Value`]: crate::Value"] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Objects"] fn call (& self , info : & Self :: TypeInfo , args : & FieldArguments < S > , executor : & Executor < Self :: Context , S > ,) -> ExecutionResult < S > ; }
};
}
