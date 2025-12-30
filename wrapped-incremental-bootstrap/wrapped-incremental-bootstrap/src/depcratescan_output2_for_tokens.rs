// Generated macro for scan_output2_for_tokens (function)
macro_rules! Depcratescan_output2_for_tokens {
() => {
// Module: crate
// Provides: {"scan_output2_for_tokens"}
// Dependencies: {}
fn scan_output2_for_tokens (tokens : & HashSet < String > , resolved : & mut HashMap < String , Vec < std :: path :: PathBuf > >) -> Result < () , Box < dyn std :: error :: Error > > { println ! ("🔍 Scanning output2 for {} tokens..." , tokens . len ()) ; for entry in fs :: read_dir ("../output2") ? { let entry = entry ? ; if entry . file_type () ? . is_dir () { let decls_dir = entry . path () . join ("src/decls") ; if decls_dir . exists () { scan_decls_for_tokens (& decls_dir , tokens , resolved) ? ; } } } Ok (()) }
};
}
