// Generated macro for debug_descriptor (function)
macro_rules! Depcrate_font_descriptordebug_descriptor {
() => {
// Module: crate::font_descriptor
// Provides: {"debug_descriptor"}
// Dependencies: {}
pub fn debug_descriptor (desc : & CTFontDescriptor) { println ! ("family: {}" , desc . family_name ()) ; println ! ("name: {}" , desc . font_name ()) ; println ! ("style: {}" , desc . style_name ()) ; println ! ("display: {}" , desc . display_name ()) ; println ! ("path: {:?}" , desc . font_path ()) ; desc . show () ; }
};
}
