// Generated macro for count (macro)
macro_rules! Depcrate_parser_sequencecount {
() => {
// Module: crate::parser::sequence
// Provides: {"count"}
// Dependencies: {}
macro_rules ! count { () => { 0 } ; ($ f : ident) => { 1 } ; ($ f : ident , $ ($ rest : ident) ,+) => { 1 + count ! ($ ($ rest) ,*) } ; }
};
}
