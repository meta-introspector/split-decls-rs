macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! decode_to_utf8_impl {
    () => {
        deps!();
        pub fn decode_to_utf8_impl (encoding : & 'static Encoding , bytes : & [u8] , expect : & str , padding : usize ,) { for i in padding .. bytes . len () { let (head , tail) = bytes . split_at (i) ; decode_to_utf8_with_boundary (encoding , head , tail , expect) ; } }
    };
}

decode_to_utf8_impl!()