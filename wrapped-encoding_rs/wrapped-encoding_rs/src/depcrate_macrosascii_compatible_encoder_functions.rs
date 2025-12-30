// Generated macro for ascii_compatible_encoder_functions (macro)
macro_rules! Depcrate_macrosascii_compatible_encoder_functions {
() => {
// Module: crate::macros
// Provides: {"ascii_compatible_encoder_functions"}
// Dependencies: {}
macro_rules ! ascii_compatible_encoder_functions { ($ bmp_body : block , $ astral_body : block , $ bmp : ident , $ astral : ident , $ slf : ident , $ source : ident , $ handle : ident , $ copy_ascii : ident , $ destination_check : ident , $ ascii_punctuation : expr) => { ascii_compatible_encoder_function ! ($ bmp_body , $ astral_body , $ bmp , $ astral , $ slf , $ source , $ handle , $ copy_ascii , $ destination_check , encode_from_utf8_raw , str , Utf8Source , $ ascii_punctuation) ; ascii_compatible_encoder_function ! ($ bmp_body , $ astral_body , $ bmp , $ astral , $ slf , $ source , $ handle , $ copy_ascii , $ destination_check , encode_from_utf16_raw , [u16] , Utf16Source , $ ascii_punctuation) ; } ; }
};
}
