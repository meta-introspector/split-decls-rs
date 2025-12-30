// Generated macro for GraphQLQuery (struct)
macro_rules! DepcrateGraphQLQuery {
() => {
// Module: crate
// Provides: {"GraphQLQuery"}
// Dependencies: {}
# [doc = " A GraphQL request which can be extracted from a query string."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```ignore"] # [doc = " #[rocket::get(\"/graphql?<query..>\")]"] # [doc = " async fn graphql_query(schema: State<'_, ExampleSchema>, query: Query) -> Result<Response, Status> {"] # [doc = "     query.execute(&schema).await"] # [doc = " }"] # [doc = " ```"] # [derive (FromForm , Debug)] pub struct GraphQLQuery { query : String , # [field (name = "operationName")] operation_name : Option < String > , variables : Option < String > , }
};
}
