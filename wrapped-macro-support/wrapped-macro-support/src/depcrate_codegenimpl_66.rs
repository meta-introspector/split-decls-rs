// Generated macro for impl_66 (impl)
macro_rules! Depcrate_codegenimpl_66 {
() => {
// Module: crate::codegen
// Provides: {"impl_66"}
// Dependencies: {}
impl TryToTokens for ast :: LinkToModule { fn try_to_tokens (& self , tokens : & mut TokenStream) -> Result < () , Diagnostic > { let mut program = TokenStream :: new () ; self . 0 . try_to_tokens (& mut program) ? ; let link_function_name = self . 0 . link_function_name (0) ; let name = Ident :: new (& link_function_name , Span :: call_site ()) ; let wasm_bindgen = & self . 0 . wasm_bindgen ; let abi_ret = quote ! { # wasm_bindgen :: convert :: WasmRet <<# wasm_bindgen :: __rt :: alloc :: string :: String as # wasm_bindgen :: convert :: FromWasmAbi >:: Abi > } ; let extern_fn = extern_fn (& name , & [] , & [] , & [] , abi_ret) ; (quote ! { { # program # extern_fn static __VAL : # wasm_bindgen :: __rt :: LazyLock <# wasm_bindgen :: __rt :: alloc :: string :: String > = # wasm_bindgen :: __rt :: LazyLock :: new (|| unsafe { <# wasm_bindgen :: __rt :: alloc :: string :: String as # wasm_bindgen :: convert :: FromWasmAbi >:: from_abi (# name () . join ()) }) ; # wasm_bindgen :: __rt :: alloc :: string :: String :: clone (& __VAL) } }) . to_tokens (tokens) ; Ok (()) } }
};
}
