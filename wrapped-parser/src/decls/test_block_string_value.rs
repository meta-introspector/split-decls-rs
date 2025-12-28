macro_rules! test_block_string_value {
    () => {
        # [test] fn test_block_string_value () { assert_eq ! (block_string_value ("") , "") ; assert_eq ! (block_string_value ("\r\n") , "") ; assert_eq ! (block_string_value ("\r\r\r\r\n\n\r\n\r\r") , "") ; assert_eq ! (block_string_value ("abc") , "abc") ; assert_eq ! (block_string_value ("line 1\r\n   line 2\n     line 3\r    line 4") , "line 1\nline 2\n  line 3\n line 4") ; assert_eq ! (block_string_value ("\r\r  some text\r\n \n \n ") , "some text") ; assert_eq ! (block_string_value (r#"
    a
    b

    c
"#) , "a\nb\n\nc") ; }
    };
}

test_block_string_value!()