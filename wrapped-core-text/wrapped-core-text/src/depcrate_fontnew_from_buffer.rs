// Generated macro for new_from_buffer (function)
macro_rules! Depcrate_fontnew_from_buffer {
() => {
// Module: crate::font
// Provides: {"new_from_buffer"}
// Dependencies: {}
pub fn new_from_buffer (buffer : & [u8]) -> Result < CTFont , () > { let ct_font_descriptor = create_font_descriptor (buffer) ? ; Ok (new_from_descriptor (& ct_font_descriptor , 16.0)) }
};
}
