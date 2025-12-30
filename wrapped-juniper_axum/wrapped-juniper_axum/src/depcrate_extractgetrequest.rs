// Generated macro for GetRequest (struct)
macro_rules! Depcrate_extractGetRequest {
() => {
// Module: crate::extract
// Provides: {"GetRequest"}
// Dependencies: {}
# [doc = " Workaround for a [`GraphQLRequest`] not being [`Deserialize`]d properly from a GET query string,"] # [doc = " containing `variables` in JSON format."] # [derive (Deserialize , Debug)] # [serde (deny_unknown_fields)] struct GetRequest { query : String , # [serde (rename = "operationName")] operation_name : Option < String > , variables : Option < String > , }
};
}
