macro_rules! parser_undefined_placeholder_in_replacement {
    () => {
        # [test] fn parser_undefined_placeholder_in_replacement () { assert_eq ! (parse_error_text ("42 ==>> $a") , "Parse error: Replacement contains undefined placeholders: $a") ; }
    };
}

parser_undefined_placeholder_in_replacement!();