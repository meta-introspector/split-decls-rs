macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! decode_without_padding_impl {
    () => {
        deps!();
        fn decode_without_padding_impl (encoding : & 'static Encoding , bytes : & [u8] , expect : & str , padding : usize ,) { decode_to_utf8_impl (encoding , bytes , expect , padding) ; decode_to_utf16_impl (encoding , bytes , & utf16_from_utf8 (expect) [..] , padding) ; decode_to_string (encoding , bytes , expect) ; }
    };
}

decode_without_padding_impl!()