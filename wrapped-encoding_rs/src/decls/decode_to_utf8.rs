macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! decode_to_utf8 {
    () => {
        deps!();
        pub fn decode_to_utf8 (encoding : & 'static Encoding , bytes : & [u8] , expect : & str) { decode_to_utf8_impl (encoding , bytes , expect , 0) ; }
    };
}

decode_to_utf8!()