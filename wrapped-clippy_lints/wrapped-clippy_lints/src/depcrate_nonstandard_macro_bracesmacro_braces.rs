// Generated macro for macro_braces (function)
macro_rules! Depcrate_nonstandard_macro_bracesmacro_braces {
() => {
// Module: crate::nonstandard_macro_braces
// Provides: {"macro_braces"}
// Dependencies: {}
fn macro_braces (conf : & [MacroMatcher]) -> FxHashMap < String , (char , char) > { let mut braces = FxHashMap :: from_iter ([("print" , ('(' , ')')) , ("println" , ('(' , ')')) , ("eprint" , ('(' , ')')) , ("eprintln" , ('(' , ')')) , ("write" , ('(' , ')')) , ("writeln" , ('(' , ')')) , ("format" , ('(' , ')')) , ("format_args" , ('(' , ')')) , ("vec" , ('[' , ']')) , ("matches" , ('(' , ')')) ,] . map (| (k , v) | (k . to_string () , v)) ,) ; for it in conf { braces . insert (it . name . clone () , it . braces) ; } braces }
};
}
