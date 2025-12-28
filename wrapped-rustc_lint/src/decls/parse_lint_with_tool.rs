macro_rules! parse_lint_with_tool {
    () => {
        # [test] fn parse_lint_with_tool () { create_default_session_globals_then (| | { assert_eq ! (parse_lint_and_tool_name ("clippy::foo") , (Some (Symbol :: intern ("clippy")) , "foo")) }) ; }
    };
}

parse_lint_with_tool!()