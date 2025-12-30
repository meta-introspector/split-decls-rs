// Generated macro for get_char (macro)
macro_rules! Depcrate_tokenizerget_char {
() => {
// Module: crate::tokenizer
// Provides: {"get_char"}
// Dependencies: {}
macro_rules ! get_char (($ me : expr , $ input : expr) => (unwrap_or_return ! ($ me . get_char ($ input) , ProcessResult :: Suspend))) ;
};
}
