// Generated macro for extern_fn (function)
macro_rules! Depcrate_codegenextern_fn {
() => {
// Module: crate::codegen
// Provides: {"extern_fn"}
// Dependencies: {}
fn extern_fn (import_name : & Ident , attrs : & [syn :: Attribute] , abi_arguments : & [TokenStream] , abi_argument_names : & [Ident] , abi_ret : TokenStream ,) -> TokenStream { quote ! { # [cfg (all (target_arch = "wasm32" , any (target_os = "unknown" , target_os = "none")))] # (# attrs) * # [link (wasm_import_module = "__wbindgen_placeholder__")] extern "C" { fn # import_name (# (# abi_arguments) ,*) -> # abi_ret ; } # [cfg (not (all (target_arch = "wasm32" , any (target_os = "unknown" , target_os = "none"))))] unsafe fn # import_name (# (# abi_arguments) ,*) -> # abi_ret { # (drop (# abi_argument_names) ;) * panic ! ("cannot call wasm-bindgen imported functions on \
                    non-wasm targets") ; } } }
};
}
