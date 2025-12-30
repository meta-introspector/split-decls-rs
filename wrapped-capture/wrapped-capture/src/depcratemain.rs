// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let force = std :: env :: args () . any (| arg | arg == "-f") ; let dest = Path :: new (env ! ("CARGO_MANIFEST_DIR")) . parent () . unwrap () . join ("workspaces") ; if ! dest . exists () { panic ! ("expected {} to exist" , dest . display ()) ; } for arg in std :: env :: args () . skip (1) . filter (| arg | ! arg . starts_with ("-")) { let source_root = fs :: canonicalize (arg) . unwrap () ; capture (& source_root , & dest , force) ; } }
};
}
