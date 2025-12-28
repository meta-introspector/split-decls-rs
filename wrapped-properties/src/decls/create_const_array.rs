macro_rules! deps {
    () => {
        EastAsianWidth!();
    };
}

macro_rules! create_const_array {
    () => {
        deps!();
        # [doc = " See [`test_enumerated_property_completeness`] for usage."] # [doc = " Example input:"] # [doc = " ```ignore"] # [doc = " impl EastAsianWidth {"] # [doc = "     pub const Neutral: EastAsianWidth = EastAsianWidth(0);"] # [doc = "     pub const Ambiguous: EastAsianWidth = EastAsianWidth(1);"] # [doc = "     ..."] # [doc = " }"] # [doc = " ```"] # [doc = " Produces `const ALL_VALUES = &[(\"Neutral\", 0u16), ...];` by"] # [doc = " explicitly casting first field of the struct to u16."] macro_rules ! create_const_array { ($ (# [$ meta : meta]) * impl $ enum_ty : ident { $ ($ (# [$ const_meta : meta]) * $ v : vis const $ i : ident : $ t : ty = $ e : expr ;) * }) => { $ (# [$ meta]) * impl $ enum_ty { $ ($ (# [$ const_meta]) * $ v const $ i : $ t = $ e ;) * # [doc = " All possible values of this enum in the Unicode version"] # [doc = " from this ICU4X release."] pub const ALL_VALUES : &'static [$ enum_ty] = & [$ ($ enum_ty ::$ i) ,*] ; } # [cfg (feature = "datagen")] impl databake :: Bake for $ enum_ty { fn bake (& self , env : & databake :: CrateEnv) -> databake :: TokenStream { env . insert ("icu_properties") ; match * self { $ (Self ::$ i => databake :: quote ! (icu_properties :: props ::$ enum_ty ::$ i) ,) * Self (v) => databake :: quote ! (icu_properties :: props ::$ enum_ty :: from_icu4c_value (# v)) , } } } impl From <$ enum_ty > for u16 { fn from (other : $ enum_ty) -> Self { other . 0 as u16 } } } }
    };
}

create_const_array!()