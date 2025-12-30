// Generated macro for ServerError (struct)
macro_rules! Depcrate_errorServerError {
() => {
// Module: crate::error
// Provides: {"ServerError"}
// Dependencies: {}
# [doc = " An error in a GraphQL server."] # [derive (Clone , Serialize , Deserialize)] pub struct ServerError { # [doc = " An explanatory message of the error."] pub message : String , # [doc = " The source of the error."] # [serde (skip)] pub source : Option < Arc < dyn Any + Send + Sync > > , # [doc = " Where the error occurred."] # [serde (skip_serializing_if = "Vec::is_empty" , default)] pub locations : Vec < Pos > , # [doc = " If the error occurred in a resolver, the path to the error."] # [serde (skip_serializing_if = "Vec::is_empty" , default)] pub path : Vec < PathSegment > , # [doc = " Extensions to the error."] # [serde (skip_serializing_if = "error_extensions_is_empty" , default)] pub extensions : Option < ErrorExtensionValues > , }
};
}
