macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! encode {
    () => {
        deps!();
        pub fn encode (encoding : & 'static Encoding , str : & str , expect : & [u8]) { let mut vec = Vec :: with_capacity (expect . len () + 32) ; let mut string = String :: with_capacity (str . len () + 32) ; let range = if cfg ! (miri) { 0usize .. 4usize } else { 0usize .. 32usize } ; for i in range { vec . clear () ; string . clear () ; for j in 0usize .. i { let c = 0x40u8 + (j as u8) ; vec . push (c) ; string . push (c as char) ; } vec . extend_from_slice (expect) ; string . push_str (str) ; encode_without_padding (encoding , & string [..] , & vec [..]) ; } }
    };
}

encode!()