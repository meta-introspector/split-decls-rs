// Generated macro for impl_77 (impl)
macro_rules! Depcrate_codegenimpl_77 {
() => {
// Module: crate::codegen
// Provides: {"impl_77"}
// Dependencies: {}
impl ToTokens for ast :: ImportStatic { fn to_tokens (& self , into : & mut TokenStream) { let ty = & self . ty ; if let Some (thread_local) = self . thread_local { thread_local_import (& self . vis , & self . rust_name , & self . wasm_bindgen , ty , ty , & self . shim , thread_local ,) . to_tokens (into) } else { let vis = & self . vis ; let name = & self . rust_name ; let wasm_bindgen = & self . wasm_bindgen ; let ty = & self . ty ; let shim_name = & self . shim ; let init = static_init (wasm_bindgen , ty , shim_name) ; into . extend (quote ! { # [automatically_derived] # [deprecated = "use with `#[wasm_bindgen(thread_local_v2)]` instead"] }) ; into . extend (quote_spanned ! { name . span () => # vis static # name : # wasm_bindgen :: JsStatic <# ty > = { fn init () -> # ty { # init } # wasm_bindgen :: __rt :: std :: thread_local ! (static _VAL : # ty = init () ;) ; # wasm_bindgen :: JsStatic { __inner : & _VAL , } } ; } ,) ; } Descriptor { ident : & self . shim , inner : quote ! { <# ty as WasmDescribe >:: describe () ; } , attrs : vec ! [] , wasm_bindgen : & self . wasm_bindgen , } . to_tokens (into) ; } }
};
}
