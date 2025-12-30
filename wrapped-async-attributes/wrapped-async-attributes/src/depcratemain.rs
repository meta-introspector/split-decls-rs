// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
# [doc = " Enables an async main function."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```ignore"] # [doc = " #[async_std::main]"] # [doc = " async fn main() -> std::io::Result<()> {"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [cfg (not (test))] # [proc_macro_attribute] pub fn main (_attr : TokenStream , item : TokenStream) -> TokenStream { let input = syn :: parse_macro_input ! (item as syn :: ItemFn) ; let ret = & input . sig . output ; let inputs = & input . sig . inputs ; let name = & input . sig . ident ; let body = & input . block ; let attrs = & input . attrs ; let vis = & input . vis ; if name != "main" { return TokenStream :: from (quote_spanned ! { name . span () => compile_error ! ("only the main function can be tagged with #[async_std::main]") , }) ; } if input . sig . asyncness . is_none () { return TokenStream :: from (quote_spanned ! { input . span () => compile_error ! ("the async keyword is missing from the function declaration") , }) ; } let result = quote ! { # vis fn main () # ret { # (# attrs) * async fn main (# inputs) # ret { # body } async_std :: task :: block_on (async { main () . await }) } } ; result . into () }
};
}
