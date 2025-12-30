// Generated macro for must_be_sync (function)
macro_rules! Depcratemust_be_sync {
() => {
// Module: crate
// Provides: {"must_be_sync"}
// Dependencies: {}
# [doc = " convert marked async code to sync code"] # [proc_macro_attribute] pub fn must_be_sync (_args : TokenStream , input : TokenStream) -> TokenStream { let mut item = parse_macro_input ! (input as Item) ; convert_sync (& mut item) . into () }
};
}
