// Generated macro for bench (function)
macro_rules! Depcratebench {
() => {
// Module: crate
// Provides: {"bench"}
// Dependencies: {}
# [doc = " Enables an async benchmark function."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```ignore"] # [doc = " #![feature(test)]"] # [doc = " extern crate test;"] # [doc = ""] # [doc = " #[async_std::bench]"] # [doc = " async fn bench_1(b: &mut test::Bencher) {"] # [doc = "     b.iter(|| {"] # [doc = "         println!(\"hello world\");"] # [doc = "     })"] # [doc = " }"] # [doc = " ```"] # [proc_macro_attribute] pub fn bench (_attr : TokenStream , item : TokenStream) -> TokenStream { let input = syn :: parse_macro_input ! (item as syn :: ItemFn) ; let ret = & input . sig . output ; let args = & input . sig . inputs ; let name = & input . sig . ident ; let body = & input . block ; let attrs = & input . attrs ; let vis = & input . vis ; if input . sig . asyncness . is_none () { return TokenStream :: from (quote_spanned ! { input . span () => compile_error ! ("the async keyword is missing from the function declaration") , }) ; } if ! args . is_empty () { return TokenStream :: from (quote_spanned ! { args . span () => compile_error ! ("async benchmarks don't take any arguments") , }) ; } let result = quote ! { # [:: core :: prelude :: v1 :: bench] # (# attrs) * # vis fn # name (b : & mut test :: Bencher) # ret { task :: block_on (task :: spawn (async { # body })) } } ; result . into () }
};
}
