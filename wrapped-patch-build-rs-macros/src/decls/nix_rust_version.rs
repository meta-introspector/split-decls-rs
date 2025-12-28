macro_rules! nix_rust_version {
    () => {
        # [proc_macro] # [decl2 (fn , name = "nix_rust_version" , vis = "pub" , hash = "4023fc4c")] pub fn nix_rust_version (input : TokenStream) -> TokenStream { mkbuildrs :: nix_rust_version_impl (input) }
    };
}

nix_rust_version!()