macro_rules! test_string_value {
    () => {
        # [test] fn test_string_value () { assert_eq ! (string_value ("abc") , "abc") ; assert_eq ! (string_value ("\\n\\b\\u2a1A") , "\n\x08\u{2A1A}") ; assert_eq ! (string_value ("\\\"\\\\") , "\"\\") ; }
    };
}

test_string_value!()