// Generated macro for quote_bind_next_or_break (macro)
macro_rules! Depcratequote_bind_next_or_break {
() => {
// Module: crate
// Provides: {"quote_bind_next_or_break"}
// Dependencies: {}
# [macro_export] # [doc (hidden)] macro_rules ! quote_bind_next_or_break { ($ var : ident) => { let $ var = match $ var . next () { Some (_x) => $ crate :: __private :: RepInterp (_x) , None => break , } ; } ; }
};
}
