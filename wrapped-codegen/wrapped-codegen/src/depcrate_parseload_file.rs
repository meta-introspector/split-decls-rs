// Generated macro for load_file (function)
macro_rules! Depcrate_parseload_file {
() => {
// Module: crate::parse
// Provides: {"load_file"}
// Dependencies: {}
fn load_file (relative_to_workspace_root : impl AsRef < Path > , features : & [Attribute] , lookup : & mut Lookup ,) -> Result < () > { let error = match do_load_file (& relative_to_workspace_root , features , lookup) . err () { None => return Ok (()) , Some (error) => error , } ; let error = error . downcast :: < Error > () ? ; let span = error . span () . start () ; bail ! (LoadFileError { path : relative_to_workspace_root . as_ref () . to_owned () , line : span . line , column : span . column + 1 , error , }) }
};
}
