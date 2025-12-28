macro_rules! parser_invalid_pattern {
    () => {
        # [test] fn parser_invalid_pattern () { assert_eq ! (parse_error_text (" ==>> ()") , "Parse error: Not a valid Rust expression, type, item, path or pattern") ; }
    };
}

parser_invalid_pattern!();