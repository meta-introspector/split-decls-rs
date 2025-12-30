// Generated macro for convert_options (function)
macro_rules! Depcrate_dbconvert_options {
() => {
// Module: crate::db
// Provides: {"convert_options"}
// Dependencies: {}
fn convert_options (opts : & [(& str , & str)]) -> Result < Vec < (CString , CString) > , Error > { opts . iter () . map (| (name , value) | { let cname = match CString :: new (name . as_bytes ()) { Ok (cname) => cname , Err (e) => return Err (Error :: new (format ! ("Invalid option name `{e}`"))) , } ; let cvalue = match CString :: new (value . as_bytes ()) { Ok (cvalue) => cvalue , Err (e) => return Err (Error :: new (format ! ("Invalid option value: `{e}`"))) , } ; Ok ((cname , cvalue)) }) . collect () }
};
}
