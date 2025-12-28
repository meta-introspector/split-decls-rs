macro_rules! test_string_prefix {
    () => {
        # [test] fn test_string_prefix () { assert_eq ! (Some ("") , string_prefix (r#""abc""#)) ; assert_eq ! (Some ("") , string_prefix (r#""""#)) ; assert_eq ! (Some ("") , string_prefix (r#"""suffix"#)) ; assert_eq ! (Some ("c") , string_prefix (r#"c"""#)) ; assert_eq ! (Some ("r") , string_prefix (r#"r"""#)) ; assert_eq ! (Some ("cr") , string_prefix (r#"cr"""#)) ; assert_eq ! (Some ("r") , string_prefix (r##"r#""#"##)) ; }
    };
}

test_string_prefix!()