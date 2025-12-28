macro_rules! deps {
    () => {
        EncoderResult!();
    };
}

macro_rules! ascii_compatible_bmp_encoder_functions {
    () => {
        deps!();
        macro_rules ! ascii_compatible_bmp_encoder_functions { ($ bmp_body : block , $ bmp : ident , $ slf : ident , $ source : ident , $ handle : ident , $ copy_ascii : ident , $ destination_check : ident , $ ascii_punctuation : expr) => { ascii_compatible_encoder_functions ! ($ bmp_body , { return (EncoderResult :: Unmappable (astral) , $ source . consumed () , $ handle . written () ,) ; } , $ bmp , astral , $ slf , $ source , $ handle , $ copy_ascii , $ destination_check , $ ascii_punctuation) ; } ; }
    };
}

ascii_compatible_bmp_encoder_functions!();