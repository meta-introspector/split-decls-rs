// Generated macro for get_completion (function)
macro_rules! Depcrate_core_config_flagsget_completion {
() => {
// Module: crate::core::config::flags
// Provides: {"get_completion"}
// Dependencies: {}
# [doc = " Returns the shell completion for a given shell, if the result differs from the current"] # [doc = " content of `path`. If `path` does not exist, always returns `Some`."] pub fn get_completion (shell : & dyn Generator , path : & Path) -> Option < String > { let mut cmd = Flags :: command () ; let current = if ! path . exists () { String :: new () } else { std :: fs :: read_to_string (path) . unwrap_or_else (| _ | { eprintln ! ("couldn't read {}" , path . display ()) ; crate :: exit ! (1) }) } ; let mut buf = Vec :: new () ; let (bin_name , _) = path . file_name () . expect ("path should be a regular file") . to_str () . expect ("file name should be UTF-8") . rsplit_once ('.') . expect ("file name should have an extension") ; cmd . set_bin_name (bin_name) ; cmd . build () ; shell . generate (& cmd , & mut buf) ; if buf == current . as_bytes () { return None ; } Some (String :: from_utf8 (buf) . expect ("completion script should be UTF-8")) }
};
}
