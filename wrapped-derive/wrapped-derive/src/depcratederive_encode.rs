// Generated macro for derive_encode (function)
macro_rules! Depcratederive_encode {
() => {
// Module: crate
// Provides: {"derive_encode"}
// Dependencies: {}
# [proc_macro_derive (Encode , attributes (bincode))] pub fn derive_encode (input : proc_macro :: TokenStream) -> proc_macro :: TokenStream { derive_encode_inner (input) . unwrap_or_else (| e | e . into_token_stream ()) }
};
}
