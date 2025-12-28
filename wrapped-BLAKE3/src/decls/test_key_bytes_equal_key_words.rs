macro_rules! test_key_bytes_equal_key_words {
    () => {
        # [test] fn test_key_bytes_equal_key_words () { assert_eq ! (TEST_KEY_WORDS , crate :: platform :: words_from_le_bytes_32 (& TEST_KEY) ,) ; }
    };
}

test_key_bytes_equal_key_words!();