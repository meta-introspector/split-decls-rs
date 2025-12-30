// Generated macro for patch_rust_impl (function)
macro_rules! Depcrate_rust_nixpatch_rust_impl {
() => {
// Module: crate::rust_nix
// Provides: {"patch_rust_impl"}
// Dependencies: {}
# [decl2 (fn , name = "patch_rust_impl" , vis = "pub" , hash = "28c77f76")] pub fn patch_rust_impl (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let patch_desc = input_str . value () ; quote ! { { println ! ("cargo:warning=🔧 Would apply patch: {}" , # patch_desc) ; println ! ("cargo:warning=💡 Use nix-build to create patched Rust") ; "patch-ready" } } . into () }
};
}
