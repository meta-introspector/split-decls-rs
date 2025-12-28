macro_rules! deps {
    () => {
        BytesToHexChars!();
    };
}

macro_rules! encode_to_iter {
    () => {
        deps!();
        fn encode_to_iter < T : iter :: FromIterator < char > > (table : & 'static [u8 ; 16] , source : & [u8]) -> T { BytesToHexChars :: new (source , table) . collect () }
    };
}

encode_to_iter!()