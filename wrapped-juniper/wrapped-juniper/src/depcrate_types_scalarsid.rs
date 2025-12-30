// Generated macro for ID (struct)
macro_rules! Depcrate_types_scalarsID {
() => {
// Module: crate::types::scalars
// Provides: {"ID"}
// Dependencies: {}
# [doc = " An ID as defined by the GraphQL specification"] # [doc = ""] # [doc = " Represented as a string, but can be converted _to_ from an integer as well."] # [derive (Clone , Debug , Deref , Deserialize , Display , Eq , From , GraphQLScalar , Into , PartialEq , Serialize ,)] # [deref (forward)] # [from (Box < str >, String)] # [into (Box < str >, String)] # [graphql (parse_token (String , i32))] pub struct ID (Box < str >) ;
};
}
