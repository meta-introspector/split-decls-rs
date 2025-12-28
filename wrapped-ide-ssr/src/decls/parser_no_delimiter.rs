macro_rules! parser_no_delimiter {
    () => {
        # [test] fn parser_no_delimiter () { assert_eq ! (parse_error_text ("foo()") , "Parse error: Cannot find delimiter `==>>`") ; }
    };
}

parser_no_delimiter!();