// Generated macro for seq_parser_pattern (macro)
macro_rules! Depcrate_parser_sequenceseq_parser_pattern {
() => {
// Module: crate::parser::sequence
// Provides: {"seq_parser_pattern"}
// Dependencies: {}
# [macro_export] # [doc (hidden)] macro_rules ! seq_parser_pattern { (; $ ($ tt : tt) *) => { ($ ($ tt) *) } ; ((_ : $ first_parser : expr , $ ($ remaining : tt) +) ; $ ($ tt : tt) *) => { $ crate :: seq_parser_pattern ! (($ ($ remaining) +) ; $ ($ tt) * _ ,) } ; (($ first_field : ident : $ first_parser : expr , $ ($ remaining : tt) +) ; $ ($ tt : tt) *) => { $ crate :: seq_parser_pattern ! (($ ($ remaining) +) ; $ ($ tt) * $ first_field ,) } ; ((_ : $ first_parser : expr) ; $ ($ tt : tt) *) => { $ crate :: seq_parser_pattern ! (; $ ($ tt) * _ ,) } ; (($ first_field : ident : $ first_parser : expr) ; $ ($ tt : tt) *) => { $ crate :: seq_parser_pattern ! (; $ ($ tt) * $ first_field ,) } ; ((_ : $ first_parser : expr ,) ; $ ($ tt : tt) *) => { $ crate :: seq_parser_pattern ! (; $ ($ tt) * _ ,) } ; (($ first_field : ident : $ first_parser : expr ,) ; $ ($ tt : tt) *) => { $ crate :: seq_parser_pattern ! (; $ ($ tt) * $ first_field ,) } ; }
};
}
