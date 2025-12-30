// Generated macro for create_font_descriptor_with_data (function)
macro_rules! Depcrate_font_managercreate_font_descriptor_with_data {
() => {
// Module: crate::font_manager
// Provides: {"create_font_descriptor_with_data"}
// Dependencies: {}
pub fn create_font_descriptor_with_data (data : CFData) -> Result < CTFontDescriptor , () > { unsafe { let ct_font_descriptor_ref = CTFontManagerCreateFontDescriptorFromData (data . as_concrete_TypeRef ()) ; if ct_font_descriptor_ref . is_null () { return Err (()) ; } Ok (CTFontDescriptor :: wrap_under_create_rule (ct_font_descriptor_ref ,)) } }
};
}
