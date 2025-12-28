macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! decode_to_utf16 {
    () => {
        deps!();
        pub fn decode_to_utf16 (encoding : & 'static Encoding , bytes : & [u8] , expect : & [u16]) { decode_to_utf16_impl (encoding , bytes , expect , 0) ; }
    };
}

decode_to_utf16!()