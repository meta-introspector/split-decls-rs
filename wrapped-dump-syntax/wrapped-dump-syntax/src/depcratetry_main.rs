// Generated macro for try_main (function)
macro_rules! Depcratetry_main {
() => {
// Module: crate
// Provides: {"try_main"}
// Dependencies: {}
fn try_main () -> Result < () , Error > { let mut args = env :: args_os () ; let _ = args . next () ; let filepath = match (args . next () , args . next ()) { (Some (arg) , None) => PathBuf :: from (arg) , _ => return Err (Error :: IncorrectUsage) , } ; let code = fs :: read_to_string (& filepath) . map_err (Error :: ReadFile) ? ; let syntax = syn :: parse_file (& code) . map_err ({ | error | Error :: ParseFile { error , filepath , source_code : code , } }) ? ; println ! ("{:#?}" , syntax) ; Ok (()) }
};
}
