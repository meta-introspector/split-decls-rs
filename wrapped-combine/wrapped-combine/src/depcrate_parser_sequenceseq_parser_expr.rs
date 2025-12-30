// Generated macro for seq_parser_expr (macro)
macro_rules! Depcrate_parser_sequenceseq_parser_expr {
() => {
// Module: crate::parser::sequence
// Provides: {"seq_parser_expr"}
// Dependencies: {}
# [macro_export] # [doc (hidden)] macro_rules ! seq_parser_expr { (; $ ($ tt : tt) *) => { ($ ($ tt) *) } ; ((_ : $ first_parser : expr , $ ($ remaining : tt) +) ; $ ($ tt : tt) *) => { $ crate :: seq_parser_expr ! (($ ($ remaining) +) ; $ ($ tt) * $ first_parser ,) } ; (($ first_field : ident : $ first_parser : expr , $ ($ remaining : tt) +) ; $ ($ tt : tt) *) => { $ crate :: seq_parser_expr ! (($ ($ remaining) +) ; $ ($ tt) * $ first_parser ,) } ; ((_ : $ first_parser : expr) ; $ ($ tt : tt) *) => { ($ ($ tt) * $ first_parser ,) } ; (($ first_field : ident : $ first_parser : expr ,) ; $ ($ tt : tt) *) => { $ crate :: seq_parser_expr ! (; $ ($ tt) * $ first_parser ,) } ; ((_ : $ first_parser : expr ,) ; $ ($ tt : tt) *) => { ($ ($ tt) * $ first_parser ,) } ; (($ first_field : ident : $ first_parser : expr) ; $ ($ tt : tt) *) => { $ crate :: seq_parser_expr ! (; $ ($ tt) * $ first_parser ,) } ; }
};
}
