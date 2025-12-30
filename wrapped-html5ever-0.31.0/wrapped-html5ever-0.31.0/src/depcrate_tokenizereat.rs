// Generated macro for eat (macro)
macro_rules! Depcrate_tokenizereat {
() => {
// Module: crate::tokenizer
// Provides: {"eat"}
// Dependencies: {}
macro_rules ! eat (($ me : expr , $ input : expr , $ pat : expr) => (unwrap_or_return ! ($ me . eat ($ input , $ pat , u8 :: eq_ignore_ascii_case) , ProcessResult :: Suspend))) ;
};
}
