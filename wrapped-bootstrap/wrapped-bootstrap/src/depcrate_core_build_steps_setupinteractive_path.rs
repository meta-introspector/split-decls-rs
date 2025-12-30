// Generated macro for interactive_path (function)
macro_rules! Depcrate_core_build_steps_setupinteractive_path {
() => {
// Module: crate::core::build_steps::setup
// Provides: {"interactive_path"}
// Dependencies: {}
pub fn interactive_path () -> io :: Result < Profile > { fn abbrev_all () -> impl Iterator < Item = ((String , String) , Profile) > { ('a' ..) . zip (1 ..) . map (| (letter , number) | (letter . to_string () , number . to_string ())) . zip (Profile :: all ()) } fn parse_with_abbrev (input : & str) -> Result < Profile , String > { let input = input . trim () . to_lowercase () ; for ((letter , number) , profile) in abbrev_all () { if input == letter || input == number { return Ok (profile) ; } } input . parse () } println ! ("Welcome to the Rust project! What do you want to do with x.py?") ; for ((letter , _) , profile) in abbrev_all () { println ! ("{}) {}: {}" , letter , profile , profile . purpose ()) ; } let template = loop { print ! ("Please choose one ({}): " , abbrev_all () . map (| ((l , _) , _) | l) . collect ::< Vec < _ >> () . join ("/")) ; io :: stdout () . flush () ? ; let mut input = String :: new () ; io :: stdin () . read_line (& mut input) ? ; if input . is_empty () { eprintln ! ("EOF on stdin, when expecting answer to question.  Giving up.") ; crate :: exit ! (1) ; } break match parse_with_abbrev (& input) { Ok (profile) => profile , Err (err) => { eprintln ! ("ERROR: {err}") ; eprintln ! ("NOTE: press Ctrl+C to exit") ; continue ; } } ; } ; Ok (template) }
};
}
