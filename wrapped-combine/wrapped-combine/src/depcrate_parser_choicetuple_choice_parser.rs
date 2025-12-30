// Generated macro for tuple_choice_parser (macro)
macro_rules! Depcrate_parser_choicetuple_choice_parser {
() => {
// Module: crate::parser::choice
// Provides: {"tuple_choice_parser"}
// Dependencies: {}
macro_rules ! tuple_choice_parser { ($ head : ident) => { tuple_choice_parser_inner ! ($ head ; $ head) ; } ; ($ head : ident $ ($ id : ident) +) => { tuple_choice_parser_inner ! ($ head ; $ head $ ($ id) +) ; tuple_choice_parser ! ($ ($ id) +) ; } ; }
};
}
