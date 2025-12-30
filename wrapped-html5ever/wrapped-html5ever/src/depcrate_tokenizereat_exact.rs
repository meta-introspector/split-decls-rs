// Generated macro for eat_exact (macro)
macro_rules! Depcrate_tokenizereat_exact {
() => {
// Module: crate::tokenizer
// Provides: {"eat_exact"}
// Dependencies: {}
macro_rules ! eat_exact (($ me : expr , $ pat : expr) => (unwrap_or_return ! ($ me . eat ($ pat , u8 :: eq) , false))) ;
};
}
