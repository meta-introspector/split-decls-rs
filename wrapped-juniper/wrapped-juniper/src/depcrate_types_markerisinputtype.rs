// Generated macro for IsInputType (trait)
macro_rules! Depcrate_types_markerIsInputType {
() => {
// Module: crate::types::marker
// Provides: {"IsInputType"}
// Dependencies: {}
# [doc = " Marker trait for types which can be used as input types."] # [doc = ""] # [doc = " The GraphQL specification differentiates between input and output"] # [doc = " types. Each type which can be used as an input type should"] # [doc = " implement this trait. The specification defines enum, scalar, and"] # [doc = " input object input types."] pub trait IsInputType < S : ScalarValue > : GraphQLType < S > { # [doc = " An arbitrary function without meaning."] # [doc = ""] # [doc = " May contain compile timed check logic which ensures that types"] # [doc = " are used correctly according to the GraphQL specification."] fn mark () { } }
};
}
