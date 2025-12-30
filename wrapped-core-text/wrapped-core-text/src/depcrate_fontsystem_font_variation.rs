// Generated macro for system_font_variation (function)
macro_rules! Depcrate_fontsystem_font_variation {
() => {
// Module: crate::font
// Provides: {"system_font_variation"}
// Dependencies: {}
# [test] fn system_font_variation () { use crate :: * ; let small = new_ui_font_for_language (kCTFontSystemDetailFontType , 19. , None) ; let ps = small . postscript_name () ; let cgfont = CGFont :: from_name (& CFString :: new (& ps)) . unwrap () ; let cgfont = new_from_CGFont (& cgfont , 0.) ; let desc = cgfont . copy_descriptor () ; let vals : Vec < (CFNumber , CFNumber) > = vec ! [(CFNumber :: from (0x6f70737a) , CFNumber :: from (17.))] ; let vals_dict = CFDictionary :: from_CFType_pairs (& vals) ; let variation_attribute = unsafe { CFString :: wrap_under_get_rule (font_descriptor :: kCTFontVariationAttribute) } ; let attrs_dict = CFDictionary :: from_CFType_pairs (& [(variation_attribute , vals_dict)]) ; let ct_var_font_desc = desc . create_copy_with_attributes (attrs_dict . to_untyped ()) . unwrap () ; let attrs = ct_var_font_desc . attributes () ; let var_attr = attrs . find (CFString :: from_static_string ("NSCTFontVariationAttribute")) ; if macos_version () >= (11 , 0 , 0) { assert ! (var_attr . is_none ()) ; } else { assert ! (var_attr . is_some ()) ; } dbg ! (ct_var_font_desc) ; }
};
}
