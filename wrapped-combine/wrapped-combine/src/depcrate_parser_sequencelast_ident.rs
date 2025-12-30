// Generated macro for last_ident (macro)
macro_rules! Depcrate_parser_sequencelast_ident {
() => {
// Module: crate::parser::sequence
// Provides: {"last_ident"}
// Dependencies: {}
macro_rules ! last_ident { ($ id : ident) => { $ id } ; ($ id : ident , $ ($ rest : ident) ,+) => { last_ident ! ($ ($ rest) ,+) } ; }
};
}
