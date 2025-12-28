macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! encode_to_vec {
    () => {
        deps!();
        pub fn encode_to_vec (encoding : & 'static Encoding , string : & str , expect : & [u8]) { let (cow , _ , _) = encoding . encode (string) ; assert_eq ! (& cow [..] , expect) ; }
    };
}

encode_to_vec!();