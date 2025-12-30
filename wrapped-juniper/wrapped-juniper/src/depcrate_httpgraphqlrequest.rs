// Generated macro for GraphQLRequest (struct)
macro_rules! Depcrate_httpGraphQLRequest {
() => {
// Module: crate::http
// Provides: {"GraphQLRequest"}
// Dependencies: {}
# [doc = " The expected structure of the decoded JSON document for either POST or GET requests."] # [doc = ""] # [doc = " For POST, you can use Serde to deserialize the incoming JSON data directly"] # [doc = " into this struct - it derives Deserialize for exactly this reason."] # [doc = ""] # [doc = " For GET, you will need to parse the query string and extract \"query\","] # [doc = " \"operationName\", and \"variables\" manually."] # [derive (Clone , Debug , Deserialize , PartialEq , Serialize)] pub struct GraphQLRequest < S = DefaultScalarValue > where S : ScalarValue , { # [doc = " GraphQL query representing this request."] pub query : String , # [doc = " Optional name of the operation associated with this request."] # [serde (rename = "operationName")] pub operation_name : Option < String > , # [doc = " Optional variables to execute the GraphQL operation with."] # [serde (bound (deserialize = "InputValue<S>: Deserialize<'de>" , serialize = "InputValue<S>: Serialize" ,))] pub variables : Option < InputValue < S > > , }
};
}
