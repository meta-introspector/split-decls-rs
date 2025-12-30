// Generated macro for GetGraphQLRequest (struct)
macro_rules! DepcrateGetGraphQLRequest {
() => {
// Module: crate
// Provides: {"GetGraphQLRequest"}
// Dependencies: {}
# [derive (Deserialize , Clone , PartialEq , Debug)] # [serde (deny_unknown_fields)] struct GetGraphQLRequest { query : String , # [serde (rename = "operationName")] operation_name : Option < String > , variables : Option < String > , }
};
}
