macro_rules! parser_empty_query {
    () => {
        # [test] fn parser_empty_query () { assert_eq ! (parse_error_text ("") , "Parse error: Cannot find delimiter `==>>`") ; }
    };
}

parser_empty_query!()