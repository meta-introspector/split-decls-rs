// Generated macro for peek (macro)
macro_rules! Depcrate_tokenizerpeek {
() => {
// Module: crate::tokenizer
// Provides: {"peek"}
// Dependencies: {}
macro_rules ! peek (($ me : expr , $ input : expr) => (unwrap_or_return ! ($ me . peek ($ input) , ProcessResult :: Suspend))) ;
};
}
