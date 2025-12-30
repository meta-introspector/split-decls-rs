// Generated macro for quote_bind_next_or_break (macro)
macro_rules! Depcrate_tokensquote_bind_next_or_break {
() => {
// Module: crate::tokens
// Provides: {"quote_bind_next_or_break"}
// Dependencies: {}
# [macro_export] # [doc (hidden)] macro_rules ! quote_bind_next_or_break { ($ var : ident) => { let $ var = match $ var . next () { Some (_x) => $ crate :: tokens :: runtime :: RepInterp (_x) , None => break , } ; } ; }
};
}
