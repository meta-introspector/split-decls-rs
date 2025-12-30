// Generated macro for seq_tuple_parser_impl (macro)
macro_rules! Depcrate_parser_sequenceseq_tuple_parser_impl {
() => {
// Module: crate::parser::sequence
// Provides: {"seq_tuple_parser_impl"}
// Dependencies: {}
# [macro_export] # [doc (hidden)] macro_rules ! seq_tuple_parser_impl { (; $ ($ tt : tt) *) => { ($ ($ tt) *) } ; ((_ : $ first_parser : expr , $ ($ remaining : tt) +) ; $ ($ tt : tt) *) => { $ crate :: seq_tuple_parser_impl ! (($ ($ remaining) +) ; $ ($ tt) * $ first_parser ,) } ; (($ first_parser : expr , $ ($ remaining : tt) +) ; $ ($ tt : tt) *) => { $ crate :: seq_tuple_parser_impl ! (($ ($ remaining) +) ; $ ($ tt) * $ first_parser ,) } ; ((_ : $ first_parser : expr $ (,) ?) ; $ ($ tt : tt) *) => { $ crate :: seq_tuple_parser_impl ! (; $ ($ tt) * $ first_parser ,) } ; (($ first_parser : expr $ (,) ?) ; $ ($ tt : tt) *) => { $ crate :: seq_tuple_parser_impl ! (; $ ($ tt) * $ first_parser ,) } ; }
};
}
