// Generated macro for token_kind_to_string (function)
macro_rules! Depcrate_pprusttoken_kind_to_string {
() => {
// Module: crate::pprust
// Provides: {"token_kind_to_string"}
// Dependencies: {}
# [doc = " Print the token kind precisely, without converting `$crate` into its respective crate name."] pub fn token_kind_to_string (tok : & TokenKind) -> Cow < 'static , str > { State :: new () . token_kind_to_string (tok) }
};
}
