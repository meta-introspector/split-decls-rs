macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! decode {
    () => {
        deps!();
        pub fn decode (encoding : & 'static Encoding , bytes : & [u8] , expect : & str) { let mut vec = Vec :: with_capacity (bytes . len () + 32) ; let mut string = String :: with_capacity (expect . len () + 32) ; let range = if cfg ! (miri) { 0usize .. 4usize } else { 0usize .. 32usize } ; for i in range { vec . clear () ; string . clear () ; for j in 0usize .. i { let c = 0x40u8 + (j as u8) ; vec . push (c) ; string . push (c as char) ; } vec . extend_from_slice (bytes) ; string . push_str (expect) ; decode_without_padding_impl (encoding , & vec [..] , & string [..] , i) ; } }
    };
}

decode!();