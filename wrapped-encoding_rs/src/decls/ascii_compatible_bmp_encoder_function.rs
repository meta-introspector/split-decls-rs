macro_rules! deps {
    () => {
        EncoderResult!();
    };
}

macro_rules! ascii_compatible_bmp_encoder_function {
    () => {
        deps!();
        macro_rules ! ascii_compatible_bmp_encoder_function { ($ bmp_body : block , $ bmp : ident , $ slf : ident , $ source : ident , $ handle : ident , $ copy_ascii : ident , $ destination_check : ident , $ name : ident , $ input : ty , $ source_struct : ident , $ ascii_punctuation : expr) => { ascii_compatible_encoder_function ! ($ bmp_body , { return (EncoderResult :: Unmappable (astral) , $ source . consumed () , $ handle . written () ,) ; } , $ bmp , astral , $ slf , $ source , $ handle , $ copy_ascii , $ destination_check , $ name , $ input , $ source_struct , $ ascii_punctuation) ; } ; }
    };
}

ascii_compatible_bmp_encoder_function!()