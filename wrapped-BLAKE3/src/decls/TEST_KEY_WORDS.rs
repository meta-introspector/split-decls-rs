macro_rules! deps {
    () => {
        CVWords!();
    };
}

macro_rules! TEST_KEY_WORDS {
    () => {
        deps!();
        pub const TEST_KEY_WORDS : CVWords = [1952540791 , 1752440947 , 1816469605 , 1752394102 , 1919907616 , 1868963940 , 1919295602 , 1684956521 ,] ;
    };
}

TEST_KEY_WORDS!()