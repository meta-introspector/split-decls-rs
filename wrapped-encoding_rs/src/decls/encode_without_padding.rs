macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! encode_without_padding {
    () => {
        deps!();
        pub fn encode_without_padding (encoding : & 'static Encoding , string : & str , expect : & [u8]) { encode_from_utf8 (encoding , string , expect) ; encode_from_utf16 (encoding , & utf16_from_utf8 (string) [..] , expect) ; encode_to_vec (encoding , string , expect) ; }
    };
}

encode_without_padding!();