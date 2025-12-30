// Generated macro for impl_106 (impl)
macro_rules! Depcrate_font_collectionimpl_106 {
() => {
// Module: crate::font_collection
// Provides: {"impl_106"}
// Dependencies: {}
impl CTFontCollection { pub fn get_descriptors (& self) -> Option < CFArray < CTFontDescriptor > > { unsafe { let font_descriptors = CTFontCollectionCreateMatchingFontDescriptors (self . 0) ; if font_descriptors . is_null () { None } else { Some (CFArray :: wrap_under_get_rule (font_descriptors)) } } } }
};
}
