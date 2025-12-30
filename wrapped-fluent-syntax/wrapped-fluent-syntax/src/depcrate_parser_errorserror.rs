// Generated macro for error (macro)
macro_rules! Depcrate_parser_errorserror {
() => {
// Module: crate::parser::errors
// Provides: {"error"}
// Dependencies: {}
macro_rules ! error { ($ kind : expr , $ start : expr) => { { Err (ParserError { pos : $ start ..$ start + 1 , slice : None , kind : $ kind , }) } } ; ($ kind : expr , $ start : expr , $ end : expr) => { { Err (ParserError { pos : $ start ..$ end , slice : None , kind : $ kind , }) } } ; }
};
}
