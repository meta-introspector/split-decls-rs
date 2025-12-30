// Generated macro for seq_parser_impl (macro)
macro_rules! Depcrate_parser_sequenceseq_parser_impl {
() => {
// Module: crate::parser::sequence
// Provides: {"seq_parser_impl"}
// Dependencies: {}
# [macro_export] # [doc (hidden)] macro_rules ! seq_parser_impl { (; $ name : ident $ ($ tt : tt) *) => { $ name { $ ($ tt) * } } ; ((_ : $ first_parser : expr , $ ($ remaining : tt) +) ; $ name : ident $ ($ tt : tt) *) => { $ crate :: seq_parser_impl ! (($ ($ remaining) +) ; $ name $ ($ tt) *) } ; (($ first_field : ident : $ first_parser : expr , $ ($ remaining : tt) +) ; $ name : ident $ ($ tt : tt) *) => { $ crate :: seq_parser_impl ! (($ ($ remaining) +) ; $ name $ ($ tt) * $ first_field : $ first_field ,) } ; ((_ : $ first_parser : expr) ; $ name : ident $ ($ tt : tt) *) => { $ crate :: seq_parser_impl ! (; $ name $ ($ tt) *) } ; (($ first_field : ident : $ first_parser : expr) ; $ name : ident $ ($ tt : tt) *) => { $ crate :: seq_parser_impl ! (; $ name $ ($ tt) * $ first_field : $ first_field ,) } ; ((_ : $ first_parser : expr ,) ; $ name : ident $ ($ tt : tt) *) => { $ crate :: seq_parser_impl ! (; $ name $ ($ tt) *) } ; (($ first_field : ident : $ first_parser : expr ,) ; $ name : ident $ ($ tt : tt) *) => { $ crate :: seq_parser_impl ! (; $ name $ ($ tt) * $ first_field : $ first_field ,) } ; }
};
}
