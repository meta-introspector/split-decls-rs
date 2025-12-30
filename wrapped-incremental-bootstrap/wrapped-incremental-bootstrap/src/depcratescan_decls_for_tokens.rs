// Generated macro for scan_decls_for_tokens (function)
macro_rules! Depcratescan_decls_for_tokens {
() => {
// Module: crate
// Provides: {"scan_decls_for_tokens"}
// Dependencies: {}
fn scan_decls_for_tokens (dir : & Path , tokens : & HashSet < String > , resolved : & mut HashMap < String , Vec < std :: path :: PathBuf > >) -> Result < () , Box < dyn std :: error :: Error > > { for entry in fs :: read_dir (dir) ? { let entry = entry ? ; let path = entry . path () ; if path . is_file () && path . extension () . map_or (false , | ext | ext == "rs") { if let Ok (content) = fs :: read_to_string (& path) { for token in tokens { if content . contains (token) { resolved . entry (token . clone ()) . or_insert_with (Vec :: new) . push (path . clone ()) ; } } } } else if path . is_dir () { scan_decls_for_tokens (& path , tokens , resolved) ? ; } } Ok (()) }
};
}
