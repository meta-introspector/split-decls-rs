// Generated macro for impl_186 (impl)
macro_rules! Depcrate_font_descriptorimpl_186 {
() => {
// Module: crate::font_descriptor
// Provides: {"impl_186"}
// Dependencies: {}
impl CTFontDescriptor { fn get_string_attribute (& self , attribute : CFStringRef) -> Option < String > { unsafe { let value = CTFontDescriptorCopyAttribute (self . 0 , attribute) ; if value . is_null () { return None ; } let value = CFType :: wrap_under_create_rule (value) ; assert ! (value . instance_of ::< CFString > ()) ; let s = CFString :: wrap_under_get_rule (value . as_CFTypeRef () as CFStringRef) ; Some (s . to_string ()) } } }
};
}
