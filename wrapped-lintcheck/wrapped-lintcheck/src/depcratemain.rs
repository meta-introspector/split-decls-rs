// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { if let Ok (addr) = env :: var ("LINTCHECK_SERVER") { driver :: drive (& addr) ; } if fs :: metadata ("lintcheck/Cargo.toml") . is_err () { eprintln ! ("lintcheck needs to be run from clippy's repo root!\nUse `cargo lintcheck` alternatively.") ; std :: process :: exit (3) ; } let config = LintcheckConfig :: new () ; match config . subcommand { Some (Commands :: Diff { old , new , truncate , write_summary , }) => json :: diff (& old , & new , truncate , write_summary) , Some (Commands :: Popular { output , number }) => popular_crates :: fetch (output , number) . unwrap () , None => lintcheck (config) , } }
};
}
