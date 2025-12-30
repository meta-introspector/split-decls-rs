// Generated macro for test (function)
macro_rules! Depcratetest {
() => {
// Module: crate
// Provides: {"test"}
// Dependencies: {}
# [doc = " Enables an async test function."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```ignore"] # [doc = " #[async_std::test]"] # [doc = " async fn my_test() -> std::io::Result<()> {"] # [doc = "     assert_eq!(2 * 2, 4);"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [proc_macro_attribute] pub fn test (_attr : TokenStream , item : TokenStream) -> TokenStream { let input = syn :: parse_macro_input ! (item as syn :: ItemFn) ; let ret = & input . sig . output ; let name = & input . sig . ident ; let body = & input . block ; let attrs = & input . attrs ; let vis = & input . vis ; if input . sig . asyncness . is_none () { return TokenStream :: from (quote_spanned ! { input . span () => compile_error ! ("the async keyword is missing from the function declaration") , }) ; } let result = quote ! { # [:: core :: prelude :: v1 :: test] # (# attrs) * # vis fn # name () # ret { async_std :: task :: block_on (async { # body }) } } ; result . into () }
};
}
