// Generated macro for contains_ui_error_patterns (function)
macro_rules! Depcrate_testcontains_ui_error_patterns {
() => {
// Module: crate::test
// Provides: {"contains_ui_error_patterns"}
// Dependencies: {}
fn contains_ui_error_patterns (file_path : & Path , keep_lto_tests : bool) -> Result < bool , String > { let file = File :: open (file_path) . map_err (| error | format ! ("Failed to read `{}`: {:?}" , file_path . display () , error)) ? ; for line in BufReader :: new (file) . lines () . map_while (Result :: ok) { let line = line . trim () ; if line . is_empty () { continue ; } if ["//@ error-pattern:" , "//@ build-fail" , "//@ run-fail" , "//@ known-bug" , "-Cllvm-args" , "//~" , "thread" ,] . iter () . any (| check | line . contains (check)) { return Ok (true) ; } if ! keep_lto_tests && (line . contains ("-Clto") || line . contains ("-C lto") || line . contains ("compile-flags: -Clinker-plugin-lto")) && ! line . contains ("-Clto=thin") { return Ok (true) ; } if line . contains ("//[") && line . contains ("]~") { return Ok (true) ; } } let file_path = file_path . display () . to_string () ; if file_path . contains ("ambiguous-4-extern.rs") { eprintln ! ("nothing found for {file_path:?}") ; } if file_path . contains ("/error-emitter/") { return Ok (true) ; } Ok (false) }
};
}
