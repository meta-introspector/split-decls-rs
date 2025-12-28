macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! decode_without_padding {
    () => {
        deps!();
        pub fn decode_without_padding (encoding : & 'static Encoding , bytes : & [u8] , expect : & str) { decode_without_padding_impl (encoding , bytes , expect , 0) ; }
    };
}

decode_without_padding!();