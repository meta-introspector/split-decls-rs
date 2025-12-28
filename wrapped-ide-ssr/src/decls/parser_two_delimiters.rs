macro_rules! parser_two_delimiters {
    () => {
        # [test] fn parser_two_delimiters () { assert_eq ! (parse_error_text ("foo() ==>> a ==>> b ") , "Parse error: More than one delimiter found") ; }
    };
}

parser_two_delimiters!()