// Generated macro for ascii_compatible_bmp_encoder_function (macro)
macro_rules! Depcrate_macrosascii_compatible_bmp_encoder_function {
() => {
// Module: crate::macros
// Provides: {"ascii_compatible_bmp_encoder_function"}
// Dependencies: {}
macro_rules ! ascii_compatible_bmp_encoder_function { ($ bmp_body : block , $ bmp : ident , $ slf : ident , $ source : ident , $ handle : ident , $ copy_ascii : ident , $ destination_check : ident , $ name : ident , $ input : ty , $ source_struct : ident , $ ascii_punctuation : expr) => { ascii_compatible_encoder_function ! ($ bmp_body , { return (EncoderResult :: Unmappable (astral) , $ source . consumed () , $ handle . written () ,) ; } , $ bmp , astral , $ slf , $ source , $ handle , $ copy_ascii , $ destination_check , $ name , $ input , $ source_struct , $ ascii_punctuation) ; } ; }
};
}
