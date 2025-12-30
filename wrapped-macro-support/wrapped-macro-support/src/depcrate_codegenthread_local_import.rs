// Generated macro for thread_local_import (function)
macro_rules! Depcrate_codegenthread_local_import {
() => {
// Module: crate::codegen
// Provides: {"thread_local_import"}
// Dependencies: {}
fn thread_local_import (vis : & syn :: Visibility , name : & Ident , wasm_bindgen : & syn :: Path , actual_ty : & syn :: Type , ty : & syn :: Type , shim_name : & Ident , thread_local : ast :: ThreadLocal ,) -> TokenStream { let init = static_init (wasm_bindgen , ty , shim_name) ; match thread_local { ast :: ThreadLocal :: V1 => quote ! { # wasm_bindgen :: __rt :: std :: thread_local ! { # [automatically_derived] # [deprecated = "use with `#[wasm_bindgen(thread_local_v2)]` instead"] # vis static # name : # actual_ty = { # init } ; } } , ast :: ThreadLocal :: V2 => { quote ! { # vis static # name : # wasm_bindgen :: JsThreadLocal <# actual_ty > = { fn init () -> # actual_ty { # init } # wasm_bindgen :: __wbindgen_thread_local ! (# wasm_bindgen , # actual_ty) } ; } } } }
};
}
