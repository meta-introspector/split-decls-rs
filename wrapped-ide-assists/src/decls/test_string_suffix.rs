macro_rules! test_string_suffix {
    () => {
        # [test] fn test_string_suffix () { assert_eq ! (Some ("") , string_suffix (r#""abc""#)) ; assert_eq ! (Some ("") , string_suffix (r#""""#)) ; assert_eq ! (Some ("a") , string_suffix (r#"""a"#)) ; assert_eq ! (Some ("i32") , string_suffix (r#"""i32"#)) ; assert_eq ! (Some ("i32") , string_suffix (r#"r""i32"#)) ; assert_eq ! (Some ("i32") , string_suffix (r##"r#""#i32"##)) ; }
    };
}

test_string_suffix!();