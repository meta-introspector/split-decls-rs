// Generated macro for SchemaError (struct)
macro_rules! Depcrate_dynamic_errorSchemaError {
() => {
// Module: crate::dynamic::error
// Provides: {"SchemaError"}
// Dependencies: {}
# [doc = " An error can occur when building dynamic schema"] # [derive (Debug , thiserror :: Error , Eq , PartialEq)] # [error ("{0}")] pub struct SchemaError (pub String) ;
};
}
