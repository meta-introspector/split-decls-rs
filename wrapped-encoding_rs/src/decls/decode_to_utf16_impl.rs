macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! decode_to_utf16_impl {
    () => {
        deps!();
        pub fn decode_to_utf16_impl (encoding : & 'static Encoding , bytes : & [u8] , expect : & [u16] , padding : usize ,) { for i in padding .. bytes . len () { let (head , tail) = bytes . split_at (i) ; decode_to_utf16_with_boundary (encoding , head , tail , expect) ; } }
    };
}

decode_to_utf16_impl!()