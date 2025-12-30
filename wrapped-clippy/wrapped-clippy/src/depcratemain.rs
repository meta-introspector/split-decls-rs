// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
pub fn main () { if env :: args () . any (| a | a == "--help" || a == "-h") { show_help () ; return ; } if env :: args () . any (| a | a == "--version" || a == "-V") { show_version () ; return ; } if let Some (pos) = env :: args () . position (| a | a == "--explain") { if let Some (mut lint) = env :: args () . nth (pos + 1) { lint . make_ascii_lowercase () ; process :: exit (clippy_lints :: explain (& lint . strip_prefix ("clippy::") . unwrap_or (& lint) . replace ('-' , "_") ,)) ; } else { show_help () ; } return ; } if let Err (code) = process (env :: args () . skip (2)) { process :: exit (code) ; } }
};
}
