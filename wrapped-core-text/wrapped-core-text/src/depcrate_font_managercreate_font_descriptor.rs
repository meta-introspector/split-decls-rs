// Generated macro for create_font_descriptor (function)
macro_rules! Depcrate_font_managercreate_font_descriptor {
() => {
// Module: crate::font_manager
// Provides: {"create_font_descriptor"}
// Dependencies: {}
pub fn create_font_descriptor (buffer : & [u8]) -> Result < CTFontDescriptor , () > { let cf_data = CFData :: from_buffer (buffer) ; unsafe { let ct_font_descriptor_ref = CTFontManagerCreateFontDescriptorFromData (cf_data . as_concrete_TypeRef ()) ; if ct_font_descriptor_ref . is_null () { return Err (()) ; } Ok (CTFontDescriptor :: wrap_under_create_rule (ct_font_descriptor_ref ,)) } }
};
}
