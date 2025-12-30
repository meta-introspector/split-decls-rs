// Generated macro for pop_except_from (macro)
macro_rules! Depcrate_tokenizerpop_except_from {
() => {
// Module: crate::tokenizer
// Provides: {"pop_except_from"}
// Dependencies: {}
macro_rules ! pop_except_from (($ me : expr , $ input : expr , $ set : expr) => (unwrap_or_return ! ($ me . pop_except_from ($ input , $ set) , ProcessResult :: Suspend))) ;
};
}
