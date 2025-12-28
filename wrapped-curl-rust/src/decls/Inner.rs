macro_rules! deps {
    () => {
        Form!();
        List!();
    };
}

macro_rules! Inner {
    () => {
        deps!();
        struct Inner < H > { handle : * mut curl_sys :: CURL , header_list : Option < List > , resolve_list : Option < List > , connect_to_list : Option < List > , form : Option < Form > , error_buf : RefCell < Vec < u8 > > , handler : H , }
    };
}

Inner!()