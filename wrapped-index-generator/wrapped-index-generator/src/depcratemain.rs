// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () , Box < dyn std :: error :: Error > > { println ! ("# Output2 Index") ; println ! ("# Generated index of all declarations in output2") ; println ! () ; for entry in fs :: read_dir ("../output2") ? { let entry = entry ? ; if entry . file_type () ? . is_dir () { let crate_path = entry . path () ; let crate_name = crate_path . file_name () . unwrap () . to_string_lossy () ; println ! ("[{}]" , crate_name) ; let decls_dir = crate_path . join ("src/decls") ; if decls_dir . exists () { scan_decls (& decls_dir , & crate_name) ? ; } println ! () ; } } Ok (()) }
};
}
