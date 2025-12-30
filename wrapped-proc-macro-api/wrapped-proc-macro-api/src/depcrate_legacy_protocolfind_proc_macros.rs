// Generated macro for find_proc_macros (function)
macro_rules! Depcrate_legacy_protocolfind_proc_macros {
() => {
// Module: crate::legacy_protocol
// Provides: {"find_proc_macros"}
// Dependencies: {}
# [doc = " Finds proc-macros in a given dynamic library."] pub (crate) fn find_proc_macros (srv : & ProcMacroServerProcess , dylib_path : & AbsPath ,) -> Result < Result < Vec < (String , ProcMacroKind) > , String > , ServerError > { let request = Request :: ListMacros { dylib_path : dylib_path . to_path_buf () . into () } ; let response = send_task (srv , request) ? ; match response { Response :: ListMacros (it) => Ok (it) , _ => Err (ServerError { message : "unexpected response" . to_owned () , io : None }) , } }
};
}
