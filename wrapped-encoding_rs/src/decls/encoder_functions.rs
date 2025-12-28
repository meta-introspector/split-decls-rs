macro_rules! deps {
    () => {
        Utf8Source!();
        Utf16Source!();
    };
}

macro_rules! encoder_functions {
    () => {
        deps!();
        macro_rules ! encoder_functions { (eof = $ eof : block , body = $ body : block , self = $ slf : ident , src_consumed = $ src_consumed : ident , source = $ source : ident , dest = $ dest : ident , c = $ c : ident , destination_handle = $ destination_handle : ident , unread_handle = $ unread_handle : ident , destination_check = $ destination_check : ident) => { encoder_function ! ($ eof , $ body , $ slf , $ src_consumed , $ source , $ dest , $ c , $ destination_handle , $ unread_handle , $ destination_check , encode_from_utf8_raw , str , Utf8Source) ; encoder_function ! ($ eof , $ body , $ slf , $ src_consumed , $ source , $ dest , $ c , $ destination_handle , $ unread_handle , $ destination_check , encode_from_utf16_raw , [u16] , Utf16Source) ; } ; }
    };
}

encoder_functions!();