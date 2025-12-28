macro_rules! new_random_cnonce {
    () => {
        fn new_random_cnonce () -> String { let raw : [u8 ; 16] = rand :: random () ; hex :: encode (& raw [..]) }
    };
}

new_random_cnonce!();