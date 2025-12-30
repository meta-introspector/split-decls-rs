// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> std :: io :: Result < () > { let v = std :: env :: var ("RUST_ANALYZER_INTERNALS_DO_NOT_USE") ; if v . is_err () { eprintln ! ("This is an IDE implementation detail, you can use this tool by exporting RUST_ANALYZER_INTERNALS_DO_NOT_USE.") ; eprintln ! ("Note that this tool's API is highly unstable and may break without prior notice") ; std :: process :: exit (122) ; } let matches = Command :: new ("proc-macro-srv") . args (& [clap :: Arg :: new ("format") . long ("format") . action (clap :: ArgAction :: Set) . default_value ("json") . value_parser (clap :: builder :: EnumValueParser :: < ProtocolFormat > :: new ()) , clap :: Arg :: new ("version") . long ("version") . action (clap :: ArgAction :: SetTrue) . help ("Prints the version of the proc-macro-srv") ,]) . get_matches () ; if matches . get_flag ("version") { println ! ("rust-analyzer-proc-macro-srv {}" , version :: version ()) ; return Ok (()) ; } let & format = matches . get_one :: < ProtocolFormat > ("format") . expect ("format value should always be present") ; run (format) }
};
}
