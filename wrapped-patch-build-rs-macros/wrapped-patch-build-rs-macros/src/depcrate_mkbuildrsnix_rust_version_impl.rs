// Generated macro for nix_rust_version_impl (function)
macro_rules! Depcrate_mkbuildrsnix_rust_version_impl {
() => {
// Module: crate::mkbuildrs
// Provides: {"nix_rust_version_impl"}
// Dependencies: {}
# [decl (fn , name = "nix_rust_version_impl" , vis = "pub" , hash = "c6cb4b8f")] pub fn nix_rust_version_impl (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let version = input_str . value () ; quote ! { { println ! ("cargo:warning=🦀 Nix Rust version: {}" , # version) ; let nix_expr = format ! (r###"
# Nix expression for Rust {}
{{ pkgs ? import <nixpkgs> {{}} }}:

let
  rustVersion = "{}";
  rustSrc = pkgs.fetchFromGitHub {{
    owner = "rust-lang";
    repo = "rust";
    rev = rustVersion;
    sha256 = "0000000000000000000000000000000000000000000000000000";
  }};
in
pkgs.rustc.override {{
  version = rustVersion;
  src = rustSrc;
}}
            "### , # version , # version) ; nix_expr } } . into () }
};
}
