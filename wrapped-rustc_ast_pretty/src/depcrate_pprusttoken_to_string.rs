// Generated macro for token_to_string (function)
macro_rules! Depcrate_pprusttoken_to_string {
() => {
// Module: crate::pprust
// Provides: {"token_to_string"}
// Dependencies: {}
# [doc = " Print the token precisely, without converting `$crate` into its respective crate name."] pub fn token_to_string (token : & Token) -> Cow < 'static , str > { State :: new () . token_to_string (token) }
};
}
