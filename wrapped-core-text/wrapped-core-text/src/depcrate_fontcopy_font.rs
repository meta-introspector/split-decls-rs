// Generated macro for copy_font (function)
macro_rules! Depcrate_fontcopy_font {
() => {
// Module: crate::font
// Provides: {"copy_font"}
// Dependencies: {}
# [test] fn copy_font () { use std :: io :: Read ; let mut f = std :: fs :: File :: open ("/System/Library/Fonts/ZapfDingbats.ttf") . unwrap () ; let mut font_data = Vec :: new () ; f . read_to_end (& mut font_data) . unwrap () ; let desc = crate :: font_manager :: create_font_descriptor (& font_data) . unwrap () ; let font = new_from_descriptor (& desc , 12.) ; drop (desc) ; let desc = font . copy_descriptor () ; drop (font) ; let font = new_from_descriptor (& desc , 14.) ; assert_eq ! (font . family_name () , "Zapf Dingbats") ; }
};
}
