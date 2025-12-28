macro_rules! deps {
    () => {
        Placeholder!();
    };
}

macro_rules! parser_repeated_name {
    () => {
        deps!();
        # [test] fn parser_repeated_name () { assert_eq ! (parse_error_text ("foo($a, $a) ==>>") , "Parse error: Placeholder `$a` repeats more than once") ; }
    };
}

parser_repeated_name!()