// Generated macro for AsyncField (trait)
macro_rules! Depcrate_macros_reflectAsyncField {
() => {
// Module: crate::macros::reflect
// Provides: {"AsyncField"}
// Dependencies: {}
# [doc = " Asynchronous field of a GraphQL [object][1] or [interface][2]."] # [doc = ""] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Objects"] # [doc = " [2]: https://spec.graphql.org/October2021#sec-Interfaces"] pub trait AsyncField < S , const N : FieldName > : FieldMeta < S , N > { # [doc = " Resolves the [`Value`] of this asynchronous [`AsyncField`]."] # [doc = ""] # [doc = " The `arguments` object contains all the specified arguments, with the"] # [doc = " default values being substituted for the ones not provided by the query."] # [doc = ""] # [doc = " The `executor` can be used to drive selections into sub-[objects][1]."] # [doc = ""] # [doc = " [1]: https://spec.graphql.org/October2021#sec-Objects"] fn call < 'b > (& 'b self , info : & 'b Self :: TypeInfo , args : & 'b FieldArguments < S > , executor : & 'b Executor < Self :: Context , S > ,) -> BoxFuture < 'b , ExecutionResult < S > > ; }
};
}
