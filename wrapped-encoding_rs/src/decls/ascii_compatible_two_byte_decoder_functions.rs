macro_rules! deps {
    () => {
        Utf8Destination!();
        Utf16Destination!();
    };
}

macro_rules! ascii_compatible_two_byte_decoder_functions {
    () => {
        deps!();
        macro_rules ! ascii_compatible_two_byte_decoder_functions { (lead = $ lead : block , trail = $ trail : block , self = $ slf : ident , non_ascii = $ non_ascii : ident , byte = $ byte : ident , lead_minus_offset = $ lead_minus_offset : ident , unread_handle_trail = $ unread_handle_trail : ident , source = $ source : ident , handle = $ handle : ident , outermost = $ outermost : tt , copy_ascii = $ copy_ascii : ident , destination_check = $ destination_check : ident , ascii_punctuation = $ ascii_punctuation : expr) => { ascii_compatible_two_byte_decoder_function ! ($ lead , $ trail , $ slf , $ non_ascii , $ byte , $ lead_minus_offset , $ unread_handle_trail , $ source , $ handle , $ outermost , $ copy_ascii , $ destination_check , decode_to_utf8_raw , u8 , Utf8Destination , $ ascii_punctuation) ; ascii_compatible_two_byte_decoder_function ! ($ lead , $ trail , $ slf , $ non_ascii , $ byte , $ lead_minus_offset , $ unread_handle_trail , $ source , $ handle , $ outermost , $ copy_ascii , $ destination_check , decode_to_utf16_raw , u16 , Utf16Destination , $ ascii_punctuation) ; } ; }
    };
}

ascii_compatible_two_byte_decoder_functions!();