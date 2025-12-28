macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! decode_to_string {
    () => {
        deps!();
        pub fn decode_to_string (encoding : & 'static Encoding , bytes : & [u8] , expect : & str) { let (cow , _ , _) = encoding . decode (bytes) ; assert_eq ! (& cow [..] , expect) ; }
    };
}

decode_to_string!();