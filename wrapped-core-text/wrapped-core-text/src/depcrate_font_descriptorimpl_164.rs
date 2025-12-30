// Generated macro for impl_164 (impl)
macro_rules! Depcrate_font_descriptorimpl_164 {
() => {
// Module: crate::font_descriptor
// Provides: {"impl_164"}
// Dependencies: {}
impl TraitAccessors for CTFontTraits { fn symbolic_traits (& self) -> CTFontSymbolicTraits { unsafe { let number = self . extract_number_for_key (kCTFontSymbolicTrait) ; number . to_i64 () . unwrap () as u32 } } fn normalized_weight (& self) -> f64 { unsafe { let number = self . extract_number_for_key (kCTFontWeightTrait) ; number . to_f64 () . unwrap () } } fn normalized_width (& self) -> f64 { unsafe { let number = self . extract_number_for_key (kCTFontWidthTrait) ; number . to_f64 () . unwrap () } } fn normalized_slant (& self) -> f64 { unsafe { let number = self . extract_number_for_key (kCTFontSlantTrait) ; number . to_f64 () . unwrap () } } }
};
}
