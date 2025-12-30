// Generated macro for handle_literal (macro)
macro_rules! Depcrate_parserhandle_literal {
() => {
// Module: crate::parser
// Provides: {"handle_literal"}
// Dependencies: {}
macro_rules ! handle_literal { ($ self : ident , $ quoted : expr , $ next_state : expr) => { { let range = $ self . advance_state ($ self . idx , $ next_state) ; if ! range . is_empty () { return Ok (Some (ParsedPatternItem :: Literal { content : Cow :: Borrowed (&$ self . input [range]) , quoted : $ quoted , })) ; } else { continue ; } } } ; }
};
}
