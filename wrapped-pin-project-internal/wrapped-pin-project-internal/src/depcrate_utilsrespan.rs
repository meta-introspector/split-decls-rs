// Generated macro for respan (function)
macro_rules! Depcrate_utilsrespan {
() => {
// Module: crate::utils
// Provides: {"respan"}
// Dependencies: {}
pub (crate) fn respan < T > (node : & T , span : Span) -> T where T : ToTokens + Parse , { let tokens = node . to_token_stream () ; let respanned = respan_tokens (tokens , span) ; syn :: parse2 (respanned) . unwrap () }
};
}
