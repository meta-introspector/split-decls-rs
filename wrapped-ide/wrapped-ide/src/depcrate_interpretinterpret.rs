// Generated macro for interpret (function)
macro_rules! Depcrate_interpretinterpret {
() => {
// Module: crate::interpret
// Provides: {"interpret"}
// Dependencies: {}
pub (crate) fn interpret (db : & RootDatabase , position : FilePosition) -> String { match find_and_interpret (db , position) { Some ((duration , mut result)) => { result . push ('\n') ; format_to ! (result , "----------------------\n") ; format_to ! (result , "  Finished in {}s\n" , duration . as_secs_f32 ()) ; result } _ => "Not inside a function, const or static" . to_owned () , } }
};
}
