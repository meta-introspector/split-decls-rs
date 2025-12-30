// Generated macro for set_optional_options (function)
macro_rules! Depcrate_dbghelpset_optional_options {
() => {
// Module: crate::dbghelp
// Provides: {"set_optional_options"}
// Dependencies: {}
unsafe fn set_optional_options (dbghelp : * mut Dbghelp) -> Option < () > { unsafe { let orig = (* dbghelp) . SymGetOptions () ? () ; (* dbghelp) . SymSetOptions () ? (orig | SYMOPT_DEFERRED_LOADS) ; (* dbghelp) . SymInitializeW () ? (GetCurrentProcess () , ptr :: null_mut () , TRUE) ; let mut search_path_buf = Vec :: new () ; search_path_buf . resize (1024 , 0) ; if (* dbghelp) . SymGetSearchPathW () ? (GetCurrentProcess () , search_path_buf . as_mut_ptr () , search_path_buf . len () as _ ,) == TRUE { let len = lstrlenW (search_path_buf . as_mut_ptr ()) ; assert ! (len >= 0) ; search_path_buf . truncate (len as usize) ; } else { search_path_buf . clear () ; search_path_buf . push (utf16_char ('.')) ; search_path_buf . push (utf16_char (';')) ; } let mut search_path = SearchPath :: new (search_path_buf) ; (* dbghelp) . EnumerateLoadedModulesW64 () ? (GetCurrentProcess () , Some (enum_loaded_modules_callback) , ((& mut search_path) as * mut SearchPath) as * mut c_void ,) ; let new_search_path = search_path . finalize () ; (* dbghelp) . SymSetSearchPathW () ? (GetCurrentProcess () , new_search_path . as_ptr ()) ; } Some (()) }
};
}
