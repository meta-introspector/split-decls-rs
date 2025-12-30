// Generated macro for IsOutputType (trait)
macro_rules! Depcrate_types_markerIsOutputType {
() => {
// Module: crate::types::marker
// Provides: {"IsOutputType"}
// Dependencies: {}
# [doc = " Marker trait for types which can be used as output types."] # [doc = ""] # [doc = " The GraphQL specification differentiates between input and output"] # [doc = " types. Each type which can be used as an output type should"] # [doc = " implement this trait. The specification defines enum, scalar,"] # [doc = " object, union, and interface as output types."] pub trait IsOutputType < S : ScalarValue > : GraphQLType < S > { # [doc = " An arbitrary function without meaning."] # [doc = ""] # [doc = " May contain compile timed check logic which ensures that types"] # [doc = " are used correctly according to the GraphQL specification."] fn mark () { } }
};
}
