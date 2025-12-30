// Generated macro for impl_75 (impl)
macro_rules! Depcrate_codegenimpl_75 {
() => {
// Module: crate::codegen
// Provides: {"impl_75"}
// Dependencies: {}
impl ToTokens for DescribeImport < '_ > { fn to_tokens (& self , tokens : & mut TokenStream) { let f = match * self . kind { ast :: ImportKind :: Function (ref f) => f , ast :: ImportKind :: Static (_) => return , ast :: ImportKind :: String (_) => return , ast :: ImportKind :: Type (_) => return , ast :: ImportKind :: Enum (_) => return , } ; let argtys = f . function . arguments . iter () . map (| arg | & arg . pat_type . ty) ; let nargs = f . function . arguments . len () as u32 ; let inform_ret = match & f . js_ret { Some (ref t) => quote ! { <# t as WasmDescribe >:: describe () ; } , None if f . function . r#async => quote ! { < JsValue as WasmDescribe >:: describe () ; } , None => quote ! { < () as WasmDescribe >:: describe () ; } , } ; Descriptor { ident : & f . shim , inner : quote ! { inform (FUNCTION) ; inform (0) ; inform (# nargs) ; # (<# argtys as WasmDescribe >:: describe () ;) * # inform_ret # inform_ret } , attrs : f . function . rust_attrs . clone () , wasm_bindgen : self . wasm_bindgen , } . to_tokens (tokens) ; } }
};
}
