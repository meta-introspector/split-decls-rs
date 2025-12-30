// Generated macro for seq_tuple_extract (macro)
macro_rules! Depcrate_parser_sequenceseq_tuple_extract {
() => {
// Module: crate::parser::sequence
// Provides: {"seq_tuple_extract"}
// Dependencies: {}
# [macro_export] # [doc (hidden)] macro_rules ! seq_tuple_extract { (; ; $ name : ident ; $ ($ arg : expr) ,* $ (,) ?) => { $ name ($ ($ arg ,) *) } ; ((_ : $ first_parser : expr , $ ($ remaining : tt) +) ; ($ first_arg : expr , $ ($ arg : expr) ,*) ; $ ($ tt : tt) *) => { $ crate :: seq_tuple_extract ! (($ ($ remaining) +) ; ($ ($ arg) ,*) ; $ ($ tt) *) } ; (($ first_parser : expr , $ ($ remaining : tt) +) ; ($ first_arg : expr , $ ($ arg : expr) ,*) ; $ ($ tt : tt) *) => { $ crate :: seq_tuple_extract ! (($ ($ remaining) +) ; ($ ($ arg) ,*) ; $ ($ tt) * $ first_arg ,) } ; ((_ : $ first_parser : expr $ (,) ?) ; ($ first_arg : expr , $ ($ arg : expr) ,*) ; $ ($ tt : tt) *) => { $ crate :: seq_tuple_extract ! (; ; $ ($ tt) *) } ; (($ first_parser : expr $ (,) ?) ; ($ first_arg : expr , $ ($ arg : expr) ,*) ; $ ($ tt : tt) *) => { $ crate :: seq_tuple_extract ! (; ; $ ($ tt) * $ first_arg) } ; }
};
}
