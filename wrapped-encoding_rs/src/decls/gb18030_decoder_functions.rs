macro_rules! deps {
    () => {
        Utf8Destination!();
        Utf16Destination!();
    };
}

macro_rules! gb18030_decoder_functions {
    () => {
        deps!();
        macro_rules ! gb18030_decoder_functions { ($ first_body : block , $ second_body : block , $ third_body : block , $ fourth_body : block , $ slf : ident , $ non_ascii : ident , $ first_minus_offset : ident , $ second : ident , $ second_minus_offset : ident , $ unread_handle_second : ident , $ third : ident , $ third_minus_offset : ident , $ unread_handle_third : ident , $ fourth : ident , $ fourth_minus_offset : ident , $ unread_handle_fourth : ident , $ source : ident , $ handle : ident , $ outermost : tt) => { gb18030_decoder_function ! ($ first_body , $ second_body , $ third_body , $ fourth_body , $ slf , $ non_ascii , $ first_minus_offset , $ second , $ second_minus_offset , $ unread_handle_second , $ third , $ third_minus_offset , $ unread_handle_third , $ fourth , $ fourth_minus_offset , $ unread_handle_fourth , $ source , $ handle , $ outermost , decode_to_utf8_raw , u8 , Utf8Destination) ; gb18030_decoder_function ! ($ first_body , $ second_body , $ third_body , $ fourth_body , $ slf , $ non_ascii , $ first_minus_offset , $ second , $ second_minus_offset , $ unread_handle_second , $ third , $ third_minus_offset , $ unread_handle_third , $ fourth , $ fourth_minus_offset , $ unread_handle_fourth , $ source , $ handle , $ outermost , decode_to_utf16_raw , u16 , Utf16Destination) ; } ; }
    };
}

gb18030_decoder_functions!();