// Generated macro for static_init (function)
macro_rules! Depcrate_codegenstatic_init {
() => {
// Module: crate::codegen
// Provides: {"static_init"}
// Dependencies: {}
fn static_init (wasm_bindgen : & syn :: Path , ty : & syn :: Type , shim_name : & Ident) -> TokenStream { let abi_ret = quote ! { # wasm_bindgen :: convert :: WasmRet <<# ty as # wasm_bindgen :: convert :: FromWasmAbi >:: Abi > } ; quote ! { # [link (wasm_import_module = "__wbindgen_placeholder__")] # [cfg (all (target_arch = "wasm32" , any (target_os = "unknown" , target_os = "none")))] extern "C" { fn # shim_name () -> # abi_ret ; } # [cfg (not (all (target_arch = "wasm32" , any (target_os = "unknown" , target_os = "none"))))] unsafe fn # shim_name () -> # abi_ret { panic ! ("cannot access imported statics on non-wasm targets") } unsafe { <# ty as # wasm_bindgen :: convert :: FromWasmAbi >:: from_abi (# shim_name () . join ()) } } }
};
}
