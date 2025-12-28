macro_rules! BytesToHexChars {
    () => {
        struct BytesToHexChars < 'a > { inner : core :: slice :: Iter < 'a , u8 > , table : & 'static [u8 ; 16] , next : Option < char > , }
    };
}

BytesToHexChars!()