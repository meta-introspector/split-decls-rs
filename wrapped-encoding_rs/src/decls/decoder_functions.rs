macro_rules! deps {
    () => {
        Utf16Destination!();
        Utf8Destination!();
    };
}

macro_rules! decoder_functions {
    () => {
        deps!();
        macro_rules ! decoder_functions { (preamble = $ preamble : block , loop_preamble = $ loop_preamble : block , eof = $ eof : block , body = $ body : block , self = $ slf : ident , src_consumed = $ src_consumed : ident , dest = $ dest : ident , source = $ source : ident , byte = $ b : ident , destination_handle = $ destination_handle : ident , unread_handle = $ unread_handle : ident , destination_check = $ destination_check : ident) => { decoder_function ! ($ preamble , $ loop_preamble , $ eof , $ body , $ slf , $ src_consumed , $ dest , $ source , $ b , $ destination_handle , $ unread_handle , $ destination_check , decode_to_utf8_raw , u8 , Utf8Destination) ; decoder_function ! ($ preamble , $ loop_preamble , $ eof , $ body , $ slf , $ src_consumed , $ dest , $ source , $ b , $ destination_handle , $ unread_handle , $ destination_check , decode_to_utf16_raw , u16 , Utf16Destination) ; } ; }
    };
}

decoder_functions!()