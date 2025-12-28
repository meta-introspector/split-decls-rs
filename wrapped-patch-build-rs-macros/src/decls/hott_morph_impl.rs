macro_rules! hott_morph_impl {
    () => {
        # [decl2 (fn , name = "hott_morph_impl" , vis = "pub" , hash = "3c342609")] pub fn hott_morph_impl (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let rust_structure = input_str . value () ; quote ! { { println ! ("cargo:warning=🔮 Applying HoTT morphism") ; let hott_type = format ! ("Type rustc_ring : U₀ := Σ (crates : FinSet) (deps : crates → crates → hProp), \
                 isAutomorphic deps × hasMonsterSymmetry crates") ; let path_equiv = "rustc_expand ≃ rustc_ast via macro_expansion_path" ; let morphism = format ! ("HoTT_Morphism {{\n  type: {},\n  path: {},\n  input: '{}'\n}}" , hott_type , path_equiv , # rust_structure) ; println ! ("cargo:warning=∞ HoTT morphism computed") ; morphism } } . into () }
    };
}

hott_morph_impl!();