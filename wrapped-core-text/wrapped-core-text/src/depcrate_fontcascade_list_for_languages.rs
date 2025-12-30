// Generated macro for cascade_list_for_languages (function)
macro_rules! Depcrate_fontcascade_list_for_languages {
() => {
// Module: crate::font
// Provides: {"cascade_list_for_languages"}
// Dependencies: {}
# [cfg (feature = "mountainlion")] pub fn cascade_list_for_languages (font : & CTFont , language_pref_list : & CFArray < CFString > ,) -> CFArray < CTFontDescriptor > { unsafe { let font_collection_ref = CTFontCopyDefaultCascadeListForLanguages (font . as_concrete_TypeRef () , language_pref_list . as_concrete_TypeRef () ,) ; CFArray :: wrap_under_create_rule (font_collection_ref) } }
};
}
