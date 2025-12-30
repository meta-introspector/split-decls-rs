// Generated macro for gql_request_from_get (function)
macro_rules! Depcrategql_request_from_get {
() => {
// Module: crate
// Provides: {"gql_request_from_get"}
// Dependencies: {}
fn gql_request_from_get < S , B > (input : & str ,) -> Result < JuniperGraphQLRequest < S > , GraphQLRequestError < B > > where S : ScalarValue , B : Body , { let mut query = None ; let mut operation_name = None ; let mut variables = None ; for (key , value) in form_urlencoded :: parse (input . as_bytes ()) . into_owned () { match key . as_ref () { "query" => { if query . is_some () { return Err (invalid_err ("query")) ; } query = Some (value) } "operationName" => { if operation_name . is_some () { return Err (invalid_err ("operationName")) ; } operation_name = Some (value) } "variables" => { if variables . is_some () { return Err (invalid_err ("variables")) ; } match serde_json :: from_str :: < InputValue < S > > (& value) . map_err (GraphQLRequestError :: Variables) { Ok (parsed_variables) => variables = Some (parsed_variables) , Err (e) => return Err (e) , } } _ => continue , } } match query { Some (query) => Ok (JuniperGraphQLRequest :: new (query , operation_name , variables)) , None => Err (GraphQLRequestError :: Invalid ("'query' parameter is missing" . into () ,)) , } }
};
}
