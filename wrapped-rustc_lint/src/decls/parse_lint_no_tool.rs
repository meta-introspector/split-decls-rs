macro_rules! parse_lint_no_tool {
    () => {
        # [test] fn parse_lint_no_tool () { create_default_session_globals_then (| | { assert_eq ! (parse_lint_and_tool_name ("foo") , (None , "foo")) }) ; }
    };
}

parse_lint_no_tool!()