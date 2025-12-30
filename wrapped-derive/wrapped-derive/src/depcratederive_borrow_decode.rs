// Generated macro for derive_borrow_decode (function)
macro_rules! Depcratederive_borrow_decode {
() => {
// Module: crate
// Provides: {"derive_borrow_decode"}
// Dependencies: {}
# [proc_macro_derive (BorrowDecode , attributes (bincode))] pub fn derive_borrow_decode (input : proc_macro :: TokenStream) -> proc_macro :: TokenStream { derive_borrow_decode_inner (input) . unwrap_or_else (| e | e . into_token_stream ()) }
};
}
