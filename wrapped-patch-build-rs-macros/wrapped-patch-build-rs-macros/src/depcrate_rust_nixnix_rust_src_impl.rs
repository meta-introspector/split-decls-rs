// Generated macro for nix_rust_src_impl (function)
macro_rules! Depcrate_rust_nixnix_rust_src_impl {
() => {
// Module: crate::rust_nix
// Provides: {"nix_rust_src_impl"}
// Dependencies: {}
# [decl2 (fn , name = "nix_rust_src_impl" , vis = "pub" , hash = "ba79e3cf")] pub fn nix_rust_src_impl (_input : TokenStream) -> TokenStream { quote ! { { use std :: process :: Command ; let output = Command :: new ("nix-store") . args (& ["--query" , "--requisites" , "/nix/store/*rustc*"]) . output () . expect ("Failed to query nix store") ; let rust_store_path = String :: from_utf8_lossy (& output . stdout) . lines () . find (| line | line . contains ("rustc") && line . contains ("src")) . unwrap_or ("/nix/store/rustc-src-not-found") ; println ! ("cargo:warning=🦀 Found Rust source: {}" , rust_store_path) ; rust_store_path . to_string () } } . into () }
};
}
