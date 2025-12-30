// Generated macro for eat (macro)
macro_rules! Depcrate_tokenizereat {
() => {
// Module: crate::tokenizer
// Provides: {"eat"}
// Dependencies: {}
macro_rules ! eat (($ me : expr , $ pat : expr) => (unwrap_or_return ! ($ me . eat ($ pat , u8 :: eq_ignore_ascii_case) , false))) ;
};
}
