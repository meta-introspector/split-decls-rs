// Generated macro for unerase_de (function)
macro_rules! Depcrate_errorunerase_de {
() => {
// Module: crate::error
// Provides: {"unerase_de"}
// Dependencies: {}
pub (crate) fn unerase_de < E : serde :: de :: Error > (e : Error) -> E { e . as_serde_de_error () }
};
}
