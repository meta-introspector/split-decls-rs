macro_rules! deps {
    () => {
        Utf16Destination!();
        Utf8Destination!();
    };
}

macro_rules! euc_jp_decoder_functions {
    () => {
        deps!();
        macro_rules ! euc_jp_decoder_functions { ($ jis0802_trail_body : block , $ jis0812_lead_body : block , $ jis0812_trail_body : block , $ half_width_katakana_body : block , $ slf : ident , $ non_ascii : ident , $ jis0208_lead_minus_offset : ident , $ byte : ident , $ unread_handle_trail : ident , $ jis0212_lead_minus_offset : ident , $ lead : ident , $ unread_handle_jis0212 : ident , $ source : ident , $ handle : ident) => { euc_jp_decoder_function ! ($ jis0802_trail_body , $ jis0812_lead_body , $ jis0812_trail_body , $ half_width_katakana_body , $ slf , $ non_ascii , $ jis0208_lead_minus_offset , $ byte , $ unread_handle_trail , $ jis0212_lead_minus_offset , $ lead , $ unread_handle_jis0212 , $ source , $ handle , decode_to_utf8_raw , u8 , Utf8Destination) ; euc_jp_decoder_function ! ($ jis0802_trail_body , $ jis0812_lead_body , $ jis0812_trail_body , $ half_width_katakana_body , $ slf , $ non_ascii , $ jis0208_lead_minus_offset , $ byte , $ unread_handle_trail , $ jis0212_lead_minus_offset , $ lead , $ unread_handle_jis0212 , $ source , $ handle , decode_to_utf16_raw , u16 , Utf16Destination) ; } ; }
    };
}

euc_jp_decoder_functions!()