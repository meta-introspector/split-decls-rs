// Generated macro for impl_78 (impl)
macro_rules! Depcrate_codegenimpl_78 {
() => {
// Module: crate::codegen
// Provides: {"impl_78"}
// Dependencies: {}
impl ToTokens for ast :: ImportString { fn to_tokens (& self , into : & mut TokenStream) { let js_sys = & self . js_sys ; let actual_ty : syn :: Type = parse_quote ! (# js_sys :: JsString) ; thread_local_import (& self . vis , & self . rust_name , & self . wasm_bindgen , & actual_ty , & self . ty , & self . shim , self . thread_local ,) . to_tokens (into) ; } }
};
}
