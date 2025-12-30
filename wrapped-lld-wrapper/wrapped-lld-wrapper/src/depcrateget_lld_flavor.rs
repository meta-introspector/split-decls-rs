// Generated macro for get_lld_flavor (function)
macro_rules! Depcrateget_lld_flavor {
() => {
// Module: crate
// Provides: {"get_lld_flavor"}
// Dependencies: {}
# [doc = " Extract LLD flavor name from the lld-wrapper executable name."] fn get_lld_flavor (current_exe_path : & Path) -> Result < & 'static str , String > { let file = current_exe_path . file_name () ; let stem = file . and_then (| s | s . to_str ()) . map (| s | s . trim_end_matches (EXE_SUFFIX)) ; Ok (match stem { Some ("ld.lld") => "gnu" , Some ("ld64.lld") => "darwin" , Some ("lld-link") => "link" , Some ("wasm-ld") => "wasm" , _ => return Err (format ! ("{:?}" , file)) , }) }
};
}
