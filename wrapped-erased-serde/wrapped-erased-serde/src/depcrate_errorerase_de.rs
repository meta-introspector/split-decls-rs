// Generated macro for erase_de (function)
macro_rules! Depcrate_errorerase_de {
() => {
// Module: crate::error
// Provides: {"erase_de"}
// Dependencies: {}
pub (crate) fn erase_de < E : serde :: de :: Error > (e : E) -> Error { serde :: de :: Error :: custom (e) }
};
}
