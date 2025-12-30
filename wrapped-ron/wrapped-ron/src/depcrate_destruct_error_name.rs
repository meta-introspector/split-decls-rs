// Generated macro for struct_error_name (function)
macro_rules! Depcrate_destruct_error_name {
() => {
// Module: crate::de
// Provides: {"struct_error_name"}
// Dependencies: {}
fn struct_error_name (error : Error , name : Option < & str >) -> Error { match error { Error :: NoSuchStructField { expected , found , outer : None , } => Error :: NoSuchStructField { expected , found , outer : name . map (ToOwned :: to_owned) , } , Error :: MissingStructField { field , outer : None } => Error :: MissingStructField { field , outer : name . map (ToOwned :: to_owned) , } , Error :: DuplicateStructField { field , outer : None } => Error :: DuplicateStructField { field , outer : name . map (ToOwned :: to_owned) , } , e => e , } }
};
}
